mod business_components;
mod database;
mod message;
mod ui_components;

use crate::component::Component;
use crate::components::{Components, CurrentComponent};
use iced::{
    widget::{button, column, container, row, text, Column, Text},
    Element, Settings, Task,
};

struct Crm {
    current_component: CurrentComponent,
    business_components: Option<Components>,
    ui_components: Option,
}

impl Crm {
    fn setup() -> (Self, Task<Message>) {
        (
            Self {
                current_component: CurrentComponent::Home,
                components: None,
            },
            Task::perform(Components::new(), |components| {
                Message::InitializeComponents(components)
            }),
        )
    }
    fn title(&self) -> String {
        String::from("CRM")
    }
    fn view(&self) -> Element<'_, Message> {
        if self.components.is_none() {
            column![container("loading")].into()
        } else {
            let components = self.components.clone().unwrap();
            match self.current_component {
                CurrentComponent::Home => {
                    let home_component = components.home;
                }
            }
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializeComponents(components) => {
                self.components = Some(components);
                Task::done(Message::InitializeHomeComponent)
            }
            Message::InitializeHomeComponent => {
                let home_component = self.components.clone().unwrap().home;
                Task::perform(
                    async move { initialize_component::<Home>(home_component).await },
                    |home| Message::HomeComponentInitialized(home),
                )
            }
            Message::HomeComponentInitialized(home) => {
                if let Some(components) = &mut self.components {
                    components.home = home;
                }
                Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(Crm::title, Crm::update, Crm::view)
        .settings(Settings::default())
        .run_with(Crm::setup)
}
