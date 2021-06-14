use crate::ilias::api::{IliasApi, IliasError};
use regex::Regex;
use lazy_static::lazy_static;
use scraper::{ElementRef, Selector};
use std::path::PathBuf;
use tokio::fs;
use futures::future::join_all;
use async_recursion::async_recursion;

const ILIAS_ROOT: &str = "ilias.php?baseClass=ilPersonalDesktopGUI&cmd=jumpToMemberships";

lazy_static! {
    pub static ref CONTAINERS: Selector = Selector::parse(".ilContainerListItemOuter").unwrap();
    pub static ref LINK: Selector = Selector::parse(".il_ContainerItemTitle > a").unwrap();
    pub static ref PROPERTY: Selector = Selector::parse(".il_ItemProperty").unwrap();
    pub static ref IMAGE: Selector = Selector::parse(".ilListItemIcon").unwrap();
    pub static ref REF_ID: Regex = Regex::new(r"ref_id=\d+").unwrap();
    pub static ref CHARS: Regex = Regex::new(r"[^(a-zA-Z0-9äöüÄÖÜ,._ ()\-)]+").unwrap();
}

#[derive(Debug)]
pub struct IliasContainer {
    pub item_type: IliasContainerType,
    pub version: usize,
    pub uri: String,
    pub title: String,
}

#[derive(Debug)]
pub enum IliasContainerType {
    File,
    Directory,
}

impl IliasContainer {
    pub fn new(element: ElementRef) -> Option<IliasContainer> {
        Some(IliasContainer {
            item_type: IliasContainer::item_type(element)?,
            version: IliasContainer::version(element).unwrap_or(1),
            uri: IliasContainer::uri(element)?,
            title: IliasContainer::title(element)?,
        })
    }

    pub fn new_root() -> IliasContainer {
        IliasContainer {
            item_type: IliasContainerType::Directory,
            version: 1,
            uri: ILIAS_ROOT.to_string(),
            title: "".to_string(),
        }
    }

    // parsed properties
    fn item_type(element: ElementRef) -> Option<IliasContainerType> {
        let img_src = element.select(&IMAGE).last()?.value().attr("src")?;
        let start_index: usize = img_src.find("icon_")? + 5;
        let end_index = start_index + img_src[start_index..].find(".svg")?;
        match &img_src[start_index..end_index] {
            "fold" | "crs" => Some(IliasContainerType::Directory),
            "file" => Some(IliasContainerType::File),
            _ => None
        }
    }

    fn version(element: ElementRef) -> Option<usize> {
        let inner_html = element.select(&PROPERTY).nth(2)?.inner_html();
        let start_index = inner_html.find("Version: ")? + "Version: ".len();
        let end_index = start_index + inner_html[start_index..].find('&')?;
        inner_html[start_index..end_index].parse().ok()
    }

    fn uri(element: ElementRef) -> Option<String> {
        Some(element.select(&LINK).last()?.value().attr("href")?.to_string())
    }

    fn title(element: ElementRef) -> Option<String> {
        Some(element.select(&LINK).last()?.inner_html())
    }

    pub fn name(&self) -> Option<String> {
        if self.title.is_empty() {
            return Some("".to_string());
        }

        match self.item_type {
            IliasContainerType::File => Some(format!("{}{}", CHARS.replace_all(self.title.as_str(), "").to_string()
                .replace(":", "")
                .replace("'", ""), if self.version > 1 { format!("_V{}", self.version.to_string()) } else { "".to_string() })),
            IliasContainerType::Directory => Some(CHARS.replace_all(self.title.as_str(), "").to_string()
                .replace(":", "")
                .replace("'", ""))
        }
    }
}


#[async_recursion]
pub async fn sync_tree(container: IliasContainer, api: &IliasApi, mut path: PathBuf) -> Result<(), IliasError> {
    match container.item_type {
        IliasContainerType::File => {
            let name = container.name().ok_or(IliasError::ParsingFailed)?;
            let dir = fs::read_dir(&path).await;

            if let Ok(mut dir) = dir {
                while let Some(entry) = dir.next_entry().await.map_err(|_| IliasError::IOOperationFailed)? {
                    if entry.path().to_str().unwrap().contains(name.as_str()) {
                        println!("already found {}", &name);
                        return Ok(());
                    }
                }
            }

            println!("downloading {}", &name);

            api.download_file(container.uri, path, name).await?;

            Ok(())
        }
        IliasContainerType::Directory => {
            let name = container.name().ok_or(IliasError::ParsingFailed)?;
            path.push(name.clone());

            println!("indexing {}", &name);

            let children: Vec<IliasContainer> = api.get_html(container.uri.as_str()).await?
                .select(&CONTAINERS)
                .into_iter()
                .map(IliasContainer::new)
                .flatten()
                .collect();

            // println!("{}: {:#?}", path.to_str().unwrap(), &children);

            let mut futures = Vec::new();

            for container in children {
                futures.push(sync_tree(container, api, path.clone()))
            }

            join_all(futures).await.into_iter().collect::<Result<Vec<()>, IliasError>>()?;

            println!("indexed {}", &name);

            Ok(())
        }
    }
}

