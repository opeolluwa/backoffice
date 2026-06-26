use serde::Serialize;

#[derive(Serialize)]
pub struct EmailRequest {
    pub from: From,
    pub to: Vec<Recipient>,
    pub subject: String,
    pub htmlbody: String,
}

#[derive(Serialize)]
pub struct From {
    pub address: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct Recipient {
    pub email_address: EmailAddress,
}

#[derive(Serialize)]
pub struct EmailAddress {
    pub address: String,
    pub name: String,
}

#[derive(Clone)]
pub struct ZeptoMail {
    client: reqwest::Client,
    api_key: String,
}

impl ZeptoMail {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: format!("Zoho-enczapikey {}", api_key.into()),
        }
    }

    pub async fn send_email(
        &self,
        email_request: EmailRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .post("https://api.zeptomail.com/v1.1/email")
            .header("Content-Type", "application/json")
            .header("Authorization", &self.api_key)
            .json(&email_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to send email: {}", error_text).into());
        }

        Ok(())
    }
}

pub struct EmailRequestBuilder {
    from: Option<From>,
    to: Vec<Recipient>,
    subject: Option<String>,
    htmlbody: Option<String>,
}

impl EmailRequestBuilder {
    pub fn new() -> Self {
        Self {
            from: None,
            to: Vec::new(),
            subject: None,
            htmlbody: None,
        }
    }

    pub fn from(mut self, address: impl Into<String>, name: impl Into<String>) -> Self {
        self.from = Some(From {
            address: address.into(),
            name: name.into(),
        });
        self
    }

    pub fn to(mut self, address: impl Into<String>, name: impl Into<String>) -> Self {
        self.to.push(Recipient {
            email_address: EmailAddress {
                address: address.into(),
                name: name.into(),
            },
        });
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn html_body(mut self, body: impl Into<String>) -> Self {
        self.htmlbody = Some(body.into());
        self
    }

    pub fn build(self) -> EmailRequest {
        EmailRequest {
            from: self.from.expect("`from` is required"),
            to: self.to,
            subject: self.subject.expect("`subject` is required"),
            htmlbody: self.htmlbody.expect("`htmlbody` is required"),
        }
    }
}
