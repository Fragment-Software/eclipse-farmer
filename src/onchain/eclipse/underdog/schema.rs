use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct CollectionsBody {
    account: String,
    name: String,
    image: String,
    description: String,
    #[serde(rename = "externalUrl")]
    external_url: String,
    soulbound: bool,
    transferable: bool,
    burnable: bool,
}

impl CollectionsBody {
    pub fn account(mut self, account: &str) -> Self {
        self.account = account.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn image(mut self, image: &str) -> Self {
        self.image = image.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn external_url(mut self, external_url: &str) -> Self {
        self.external_url = external_url.to_string();
        self
    }

    pub fn soulbound(mut self, soulbound: bool) -> Self {
        self.soulbound = soulbound;
        self
    }

    pub fn transferable(mut self, transferable: bool) -> Self {
        self.transferable = transferable;
        self
    }

    pub fn burnable(mut self, burnable: bool) -> Self {
        self.burnable = burnable;
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct CollectionsResponse {
    pub transaction: String,
}
