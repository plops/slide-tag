use crate::types::{ActionabilityError, AutoWaitOptions, ElementState};
use crate::waiter::wait_for_states;
use chromiumoxide::Page;
use std::future::Future;
use std::pin::Pin;

pub trait PageAutoWaitExt {
    fn auto_click<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>>;
    fn auto_fill<'a>(&'a self, selector: &'a str, text: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>>;
    fn auto_wait_visible<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>>;
}

impl PageAutoWaitExt for Page {
    fn auto_click<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>> {
        Box::pin(async move {
            let opts = AutoWaitOptions::default();
            let states = [ElementState::Visible, ElementState::Stable, ElementState::Enabled];
            wait_for_states(self, selector, &states, &opts).await?;
            
            let el = self.find_element(selector).await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            el.click().await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            Ok(())
        })
    }

    fn auto_fill<'a>(&'a self, selector: &'a str, text: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>> {
        Box::pin(async move {
            let opts = AutoWaitOptions::default();
            // Fill braucht Visible, Enabled, Editable
            let states = [ElementState::Visible, ElementState::Enabled, ElementState::Editable];
            wait_for_states(self, selector, &states, &opts).await?;
            
            let el = self.find_element(selector).await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            // Click to focus, then clear and type
            el.click().await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            self.evaluate(format!("document.querySelector(`{selector}`).value = ''")).await.ok();
            el.type_str(text).await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            Ok(())
        })
    }

    fn auto_wait_visible<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>> {
        Box::pin(async move {
            let opts = AutoWaitOptions::default();
            wait_for_states(self, selector, &[ElementState::Visible], &opts).await
        })
    }
}
