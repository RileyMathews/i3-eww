use std::process::exit;

use i3ipc::{event::Event, I3Connection, I3EventListener, Subscription};

struct Workspace {
    number: i32,
    urgent: bool,
    focused: bool,
}

///	builds the i3 workspace widget and prints it
pub fn build_widget(connection: &mut I3Connection, display_name: &str) {
    //	open base element
    let base = "(box :class \"workspace\" :orientation \"h\" :spacing 2 :space-evenly false ";
    let mut output = base.to_string();

    //	get workspaces from IPC
    let reply = connection.get_workspaces();
    if reply.is_err() {
        exit(1);
    }
    let workspaces_resposne = reply.ok().unwrap();

    let mut workspaces: Vec<Workspace> = workspaces_resposne
        .workspaces
        .iter()
        .filter(|ws| ws.output == display_name)
        .map(|ws| Workspace {
            number: ws.num,
            urgent: ws.urgent,
            focused: ws.focused,
        })
        .collect();

    workspaces.sort_by(|a, b| a.number.cmp(&b.number));

    //	loop to build elements for workspaces
    for ws in workspaces {
        //	build classes
        let mut classes = String::from("ws-btn ");
        if ws.focused {
            classes += "focused ";
        }
        if ws.urgent {
            classes += "urgent ";
        }
        //	build workspace number
        let ws_num = ws.number.to_string();

        //	build element yuck
        let element = format!("(button :vexpand true :class \"{classes}\" :onclick \"i3-msg workspace {ws_num}\" \"{ws_num}\")");
        //	... and add to output
        output += &element;
    }

    //	... and emit!
    println!("{output})");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let display_name = &args[1];
    //	open IPC
    let mut connection = I3Connection::connect().unwrap();
    //	build initial widget
    build_widget(&mut connection, &display_name);

    //	and await workspace and mode events effectively forever
    let mut listener = I3EventListener::connect().unwrap();
    let subs = [
        Subscription::Workspace,
        Subscription::Mode,
        Subscription::Window,
    ];
    listener.subscribe(&subs).unwrap();
    for event in listener.listen() {
        match event.unwrap() {
            Event::WorkspaceEvent(_) => build_widget(&mut connection, &display_name),
            _ => (),
        }
    }
}
