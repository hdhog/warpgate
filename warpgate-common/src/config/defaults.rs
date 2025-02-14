use std::net::ToSocketAddrs;
use std::time::Duration;

use crate::{ListenEndpoint, Secret};

pub(crate) const fn _default_true() -> bool {
    true
}

pub(crate) const fn _default_false() -> bool {
    false
}

pub(crate) const fn _default_ssh_port() -> u16 {
    22
}

pub(crate) const fn _default_mysql_port() -> u16 {
    3306
}

#[inline]
pub(crate) fn _default_username() -> String {
    "root".to_owned()
}

#[inline]
pub(crate) fn _default_empty_string() -> String {
    "".to_owned()
}

#[inline]
pub(crate) fn _default_recordings_path() -> String {
    "./data/recordings".to_owned()
}

#[inline]
pub(crate) fn _default_database_url() -> Secret<String> {
    Secret::new("sqlite:data/db".to_owned())
}

#[inline]
pub(crate) fn _default_http_listen() -> ListenEndpoint {
    #[allow(clippy::unwrap_used)]
    ListenEndpoint("0.0.0.0:8888".to_socket_addrs().unwrap().next().unwrap())
}

#[inline]
pub(crate) fn _default_mysql_listen() -> ListenEndpoint {
    #[allow(clippy::unwrap_used)]
    ListenEndpoint("0.0.0.0:33306".to_socket_addrs().unwrap().next().unwrap())
}

#[inline]
pub(crate) fn _default_retention() -> Duration {
    Duration::SECOND * 60 * 60 * 24 * 7
}

#[inline]
pub(crate) fn _default_empty_vec<T>() -> Vec<T> {
    vec![]
}

pub(crate) fn _default_ssh_listen() -> ListenEndpoint {
    #[allow(clippy::unwrap_used)]
    ListenEndpoint("0.0.0.0:2222".to_socket_addrs().unwrap().next().unwrap())
}

pub(crate) fn _default_ssh_keys_path() -> String {
    "./data/keys".to_owned()
}

#[inline]
pub(crate) fn _default_external_proxy_header_host() -> String {
    "host".to_owned()
}

#[inline]
pub(crate) fn _default_external_proxy_header_proto() -> String {
    "x-forwarded-proto".to_owned()
}
