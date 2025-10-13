
use  lychee_lib::*;
#[tokio::main]
async fn main(){
    let response = lychee_lib::check("../../content/posts/Obsidian Blogging Workflow/Obsidian Blogging Workflow.md").await.unwrap();
    dbg!("{}", response);
}
