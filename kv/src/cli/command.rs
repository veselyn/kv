use crate::app::App;

pub trait Execute {
    async fn execute(self, app: App) -> anyhow::Result<()>;
}
