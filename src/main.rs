use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::iced_runtime::core::window;
use cosmic::Element;
use std::time::Duration;

const ID: &str = "com.system76.CosmicAppletCpuTemp";
const UPDATE_INTERVAL_SECS: u64 = 2;

pub struct Window {
    core: Core,
    temperature: f32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            core: Core::default(),
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
        let window = Window {
            core,
            temperature: read_cpu_temp().unwrap_or(0.0),
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
                if let Ok(temp) = read_cpu_temp() {
                    self.temperature = temp;
                }
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

fn read_cpu_temp() -> Result<f32, Box<dyn std::error::Error>> {
    if let Ok(output) = std::process::Command::new("sensors")
        .arg("-j")
        .output()
    {
        if let Ok(json_str) = String::from_utf8(output.stdout) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(k10temp) = json.get("k10temp-pci-00c3") {
                    if let Some(tctl) = k10temp.get("Tctl") {
                        if let Some(temp) = tctl.get("temp1_input").and_then(|v| v.as_f64()) {
                            return Ok(temp as f32);
                        }
                    }
                }
                for key in json.as_object().unwrap().keys() {
                    if key.contains("cpu") || key.contains("k10") || key.contains("coretemp") {
                        if let Some(chip) = json.get(key) {
                            if let Some(temp_obj) = chip.as_object() {
                                for (_, value) in temp_obj {
                                    if let Some(temp) = value.get("temp1_input")
                                        .and_then(|v| v.as_f64())
                                        .or_else(|| value.get("input").and_then(|v| v.as_f64()))
                                    {
                                        return Ok(temp as f32);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Ok(content) = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(millidegrees) = content.trim().parse::<f32>() {
            return Ok(millidegrees / 1000.0);
        }
    }

    Err("Could not read CPU temperature".into())
}

fn main() -> cosmic::iced::Result {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "warn")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    cosmic::applet::run::<Window>(())
}
