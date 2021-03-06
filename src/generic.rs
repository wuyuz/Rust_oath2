use oxide_auth::endpoint::Solicitation;
use std::fmt;

pub use crate::client::{Client, Config as ClientConfig, Error as ClientError};

/// Try to open the server url `http://localhost:8020` in the browser, or print a guiding statement
/// to the console if this is not possible.
pub fn open_in_browser() {
    use std::io::{Error, ErrorKind};
    use std::process::Command;

    let target_addres = "http://localhost:8020/";

    // As suggested by <https://stackoverflow.com/questions/3739327/launching-a-website-via-windows-commandline>
    let open_with = if cfg!(target_os = "linux") {
        // `xdg-open` chosen over `x-www-browser` due to problems with the latter (#25)
        Ok("xdg-open")
    } else if cfg!(target_os = "windows") {
        Ok("explorer")
    } else if cfg!(target_os = "macos") {
        Ok("open")
    } else {
        Err(Error::new(ErrorKind::Other, "Open not supported"))
    };

    open_with.and_then(|cmd| Command::new(cmd).arg(target_addres).status())
        .and_then(|status| if status.success() {
            Ok(())
        } else { 
            Err(Error::new(ErrorKind::Other, "Non zero status")) 
        })
        .unwrap_or_else(|_| println!("Please navigate to {}", target_addres));
}

pub fn consent_page_html(route: &str, solicitation: Solicitation) -> String {
    macro_rules! template {
        () => {
"<html>'{0:}' (at {1:}) is requesting permission for '{2:}'
<form method=\"post\">
    <input type=\"submit\" value=\"Accept\" formaction=\"{4:}?{3:}&allow=true\">
    <input type=\"submit\" value=\"Deny\" formaction=\"{4:}?{3:}&deny=true\">
</form>
</html>"
        };
    }

    let grant = solicitation.pre_grant();
    let state = solicitation.state();

    let mut extra = vec![
        ("response_type", "code"),
        ("client_id", grant.client_id.as_str()),
        ("redirect_uri", grant.redirect_uri.as_str()),
    ];

    if let Some(state) = state {
        extra.push(("state", state));
    }

    format!(template!(), 
        grant.client_id,
        grant.redirect_uri,
        grant.scope,
        serde_urlencoded::to_string(extra).unwrap(),
        &route,
    )
}