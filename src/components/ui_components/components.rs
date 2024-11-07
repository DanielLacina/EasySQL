use crate::components::business_components::{
    component::BusinessComponent, components::BusinessComponents,
};
use crate::components::ui_components::home::{events::HomeMessage, home::HomeUI};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
};
use iced::Task;

pub type HomeUIComponent = HomeUI;

pub enum ComponentsMessage {
    InitializeComponents(UIComponents),
}

impl Event for ComponentsMessage {}

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

#[derive(Debug, Clone)]
pub struct UIComponents {
    pub home_ui: HomeUIComponent,
}

impl UIComponent for UIComponents {
    type EventType = ComponentsMessage;

    async fn initialize_component(&mut self) {}
    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            ComponentsMessage::InitializeComponents(ui_components) => {
                self = Some(ui_components);
                Task::done(Message::Home(HomeMessage::InitializeComponent))
            }
        }
    }
}

impl UIComponents {
    pub async fn new() -> Self {
        let business_components = BusinessComponents::new().await;
        Self {
            home_ui: HomeUI::new(business_components.home),
        }
    }
}
