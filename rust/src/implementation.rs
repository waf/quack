#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use interface::*;

use std::collections::HashMap;
use std::env;
use config;
use failure::Error;
use slack_integration::connection::SlackConnection;

pub struct Application {
    emit: ApplicationEmitter,
    connection: SlackConnection,
    channels: Channels,
}

impl ApplicationTrait for Application {
    fn new(emit: ApplicationEmitter, channels: Channels) -> Application {
        println!("Application::new");

        let settings = read_settings("Settings.toml")
            .expect(&format!(
                "Could not find Settings.toml configuration file at path {}",
                env::current_dir().unwrap().display()
            ));

        // set up a real-time connection to slack in a background thread.
        // it will use a channel to communicate with the UI.
        let token = settings["token"].to_owned();
        let connection = SlackConnection::start_in_background(token);

        Application {
            emit: emit,
            connection: connection,
            channels: channels,
        }
    }
    fn emit(&self) -> &ApplicationEmitter {
        println!("Application::emit");
        &self.emit
    }
    fn channels(&self) -> &Channels {
        &self.channels
    }
    fn channels_mut(&mut self) -> &mut Channels {
        &mut self.channels
    }
}

#[derive(Default, Clone)]
struct ChannelsItem {
    name: String,
}

pub struct Channels {
    emit: ChannelsEmitter,
    model: ChannelsList,
    list: Vec<ChannelsItem>,
}

impl ChannelsTrait for Channels {
    fn new(emit: ChannelsEmitter, model: ChannelsList) -> Channels {
        println!("Channels::new");
        Channels {
            emit: emit,
            model: model,
            list: vec![ChannelsItem::default(); 10]
        }
    }
    fn emit(&self) -> &ChannelsEmitter {
        println!("Channels::emit");
        &self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn name(&self, index: usize) -> &str {
        &self.list[index].name
    }
}

fn read_settings(filename: &str) -> Result<HashMap<String, String>, Error> {
    let mut settings_file = config::Config::default();
    settings_file.merge(config::File::with_name(filename))?;
    let settings = settings_file.try_into::<HashMap<String, String>>()?;
    Ok(settings)
}

