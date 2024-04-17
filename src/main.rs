use std::process::{exit, Command};

use i3ipc::{
    event::{Event, ModeEventInfo},
    I3Connection, I3EventListener, Subscription,
};

///	builds the i3 workspace widget and prints it
pub fn build_widget(connection: &mut I3Connection) {
    //	open base element
    let base = "(box :class \"workspace\" :orientation \"h\" :spacing 2 :space-evenly false ";
    let mut output = base.to_string();

    //	get workspaces from IPC
    let reply = connection.get_workspaces();
    if reply.is_err() {
        exit(1);
    }
    let workspaces = reply.ok().unwrap();

    //	loop to build elements for workspaces
    for ws in workspaces.workspaces {
        //	build classes
        let mut classes = String::from("ws-btn ");
        if ws.focused {
            classes += "focused ";
        }
        if ws.urgent {
            classes += "urgent ";
        }
        //	build workspace number
        let ws_num = ws.num.to_string();

        //	build element yuck
        let element = format!("(button :vexpand true :class \"{classes}\" :onclick \"i3-msg workspace {ws_num}\" \"{ws_num}\")");
        //	... and add to output
        output += &element;
    }

    //	... and emit!
    println!("{output})");
}

///	issues a command for eww to update the WM_MODE variable on
///	i3 mode change.
pub fn set_mode(e: ModeEventInfo) {
    set_eww_variable("WM_MODE", e.change);
}

fn main() {
    //	open IPC
    let mut connection = I3Connection::connect().unwrap();
    //	build initial widget
    build_widget(&mut connection);

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
            Event::WorkspaceEvent(_) => build_widget(&mut connection),
            Event::ModeEvent(e) => set_mode(e),
            Event::WindowEvent(e) => handle_window_change(e),
            _ => unreachable!(),
        }
    }
}

fn handle_window_change(e: i3ipc::event::WindowEventInfo) {
    match e.change {
        i3ipc::event::inner::WindowChange::Focus => change_window_text(e.container.name),
        _ => (),
    };
}

fn change_window_text(name: Option<String>) {
    match name {
        Some(name) => set_eww_variable("FOCUSED_WINDOW", name),
        None => (),
    }
}

fn set_eww_variable(var: &str, value: String) {
    let mut cmd = Command::new("eww");
    let var_string = format!("{}={}", var, value);
    cmd.args(["update", &var_string]);
    cmd.output().ok();
}
