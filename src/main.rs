use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::iced_runtime::core::window;
use cosmic::Element;
use std::time::Duration;

const ID: &str = "com.system76.CosmicAppletCpuTemp";
const UPDATE_INTERVAL_SECS: u64 = 2;

use sysinfo::{Components, RefreshKind};

pub struct Window {
    core: Core,
    components: Components,
    temperature: f32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            core: Core::default(),
            components: Components::new_with_refreshed_list(),
            temperature: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
    Surface(cosmic::surface::Action),
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let mut components = Components::new_with_refreshed_list();
        let temperature = get_cpu_temp(&components);
        
        let window = Window {
            core,
            components,
            temperature,
        };
        (window, Task::none())
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(UPDATE_INTERVAL_SECS))
            .map(|_| Message::Tick)
    }

    fn on_close_requested(&self, _id: window::Id) -> Option<Message> {
        None
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.components.refresh(true);
                self.temperature = get_cpu_temp(&self.components);
                Task::none()
            }
            Message::Surface(a) => {
                cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ))
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let temp_display = format!("{:.0}°C", self.temperature);
        let tooltip_text = format!("CPU Temperature: {:.1}°C", self.temperature);
        
        let text_widget = cosmic::widget::text(temp_display)
            .size(14);
        
        let container = cosmic::widget::container(text_widget)
            .align_y(cosmic::iced::alignment::Vertical::Center)
            .height(Length::Fill)
            .padding([0, 8]);

        Element::from(
            self.core.applet.applet_tooltip::<Message>(
                container,
                tooltip_text,
                false,
                |a| Message::Surface(a),
                None,
            )
        )
    }

    fn view_window(&self, _id: Id) -> Element<'_, Message> {
        "CPU Temperature".into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

fn get_cpu_temp(components: &Components) -> f32 {
    // Try to find a component that looks like a CPU package first
    let mut cpu_temp = 0.0;
    let mut found_package = false;

    for component in components {
        let label = component.label().to_lowercase();
        
        // Priority to Package id 0 or generic k10temp (AMD) / coretemp (Intel) package
        if label.contains("package id 0") || label == "tctl" {
            cpu_temp = component.temperature();
            found_package = true;
            break;
        }

        // Fallback or accumulate if we want average (here keeping simple: max or first found cpu-ish thing)
        if !found_package && (label.contains("cpu") || label.contains("core")) {
             // If we haven't found a package temp yet, take the highest core temp seen so far
             let t = component.temperature();
             if t > cpu_temp {
                 cpu_temp = t;
             }
        }
    }
    
    // If still 0, just take the first thing that has a temp > 0 as a hail mary
    if cpu_temp == 0.0 && !components.is_empty() {
        for component in components {
            let t = component.temperature();
            if t > 0.0 {
                cpu_temp = t;
                break;
            }
        }
    }

    cpu_temp
}

fn main() -> cosmic::iced::Result {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "warn")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    cosmic::applet::run::<Window>(())
}
