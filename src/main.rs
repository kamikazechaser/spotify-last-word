use mpris::PlayerFinder;
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

fn main() {
    let all_players = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_all()
        .expect("Could not find any active player");

    let mut handler = SinkController::create().unwrap();
    let mut mute_status = false;

    for player in all_players {
        if player.identity() == "Spotify" {
            let events = player.events().expect("Could not start event stream");

            for event in events {
                match event {
                    Ok(event) => {
                        if let mpris::Event::TrackChanged(event) = event {
                            let title = event.title().unwrap_or_default();
                            println!("{}", title);

                            if title == "Advertisement" {
                                // TODO: device index 0 is hardocoded here
                                handler.set_device_mute_by_index(0, true);
                                mute_status = true;
                                println!("Muting ad");
                            } else {
                                if mute_status {
                                    handler.set_device_mute_by_index(0, false);
                                    mute_status = false;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("D-Bus error: {}. Aborting.", err);
                        break;
                    }
                }
            }
        } else {
            panic!("Is Spotify desktop running?")
        }
    }
}
