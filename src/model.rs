use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct WindowId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct TabId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct OsWindowId(pub u32);

pub struct Pid(u32);

pub enum Matcher {
    Id(WindowId)
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LsOutput(pub Vec<OsWindow>);

impl IntoIterator for LsOutput {
    type Item = OsWindow;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct OsWindow {
    pub id: OsWindowId,
    pub is_active: bool,
    pub is_focused: bool,
    pub tabs: Vec<Tab>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Tab {
    pub id: TabId,
    pub is_active: bool,
    pub is_focused: bool,
    pub windows: Vec<Window>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Window {
    pub id: WindowId,
    pub is_active: bool,
    pub is_focused: bool,
    pub foreground_processes: Vec<Process>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Process {
    pub pid: u32,
    pub cwd: String,
    pub cmdline: Vec<String>,
}

#[cfg(test)]
mod test {
    use lazy_static::__Deref;
    use pretty_assertions::assert_eq;

    use super::{test_fixture::LS_OUTPUT, test_fixture::LS_OUTPUT_JSON, LsOutput};

    #[test]
    fn test_kitty_ls_output_can_be_deserialized() {
        let output: LsOutput = serde_json::from_str(LS_OUTPUT_JSON).unwrap();

        let expected: &LsOutput = LS_OUTPUT.deref();

        assert_eq!(&output, expected);
    }
}

#[cfg(test)]
pub mod test_fixture {
    use lazy_static::lazy_static;

    use super::{LsOutput, OsWindow, OsWindowId, Process, Tab, TabId, Window, WindowId};

    lazy_static! {
    pub static ref LS_OUTPUT: LsOutput = LsOutput(
        vec![
            OsWindow {
                id: OsWindowId(1u32),
                is_active: true,
                is_focused: true,
                tabs: vec![
                    Tab {
                        id: TabId(1u32),
                        is_active: true,
                        is_focused: true,
                        windows: vec![
                            Window {
                                id: WindowId(1u32),
                                is_active: false,
                                is_focused: false,
                                foreground_processes: vec![
                                Process {
                                    cmdline: vec![
                                      "/nix/store/6z1v4fzjw416c38j4013y9wam07q5zbs-rust-default-1.73.0/libexec/rust-analyzer-proc-macro-srv".to_string()
                                    ],
                                    cwd: "/path/to/felis".to_string(),
                                    pid: 40339
                                },
                                Process {
                                  cmdline: vec![
                                    "/nix/store/0g95h72qqdxlig31n6ahcz1ch1jsg9q4-rust-analyzer-unwrapped-2023-05-15/bin/rust-analyzer".to_string()
                                  ],
                                  cwd: "/path/to/felis".to_string(),
                                  pid: 38646
                                },
                                Process {
                                  cmdline: vec![
                                    "/etc/profiles/per-user/gaborpihaj/bin/hx".to_string()
                                  ],
                                  cwd: "/path/to/felis".to_string(),
                                  pid: 38411
                              }],
                            },
                            Window {
                                id: WindowId(2u32),
                                is_active: true,
                                is_focused: true,
                                foreground_processes: vec![
                                    Process {
                                        pid: 49915,
                                        cwd: "/path/to/felis".to_string(),
                                        cmdline: vec![
                                            "kitten".to_string(),
                                            "@".to_string(),
                                            "ls".to_string(),
                                        ],
                                    },
                                ]
                            },
                            Window {
                                id: WindowId(3u32),
                                is_active: false,
                                is_focused: false,
                                foreground_processes: vec![
                                    Process {
                                        pid: 983,
                                        cwd: "/path/to/felis".to_string(),
                                        cmdline: vec![
                                            "-zsh".to_string(),
                                        ],
                                    },
                                ],
                            }
                        ],
                    }
                ],
            }
        ]
    );
    }

    pub static LS_OUTPUT_JSON: &str = r#"[
{
    "id": 1,
    "is_active": true,
    "is_focused": true,
    "last_focused": true,
    "platform_window_id": 130,
    "tabs": [
      {
        "active_window_history": [
          3,
          2,
          1
        ],
        "enabled_layouts": [
          "fat",
          "grid",
          "horizontal",
          "splits",
          "stack",
          "tall",
          "vertical"
        ],
        "id": 1,
        "is_active": true,
        "is_focused": true,
        "layout": "grid",
        "layout_opts": {},
        "layout_state": {
          "biased_cols": {},
          "biased_rows": {}
        },
        "title": "kitty @ ls",
        "windows": [
          {
            "cmdline": [
              "-zsh"
            ],
            "columns": 119,
            "cwd": "/path/to/felis",
            "env": {},
            "foreground_processes": [
              {
                "cmdline": [
                  "/nix/store/6z1v4fzjw416c38j4013y9wam07q5zbs-rust-default-1.73.0/libexec/rust-analyzer-proc-macro-srv"
                ],
                "cwd": "/path/to/felis",
                "pid": 40339
              },
              {
                "cmdline": [
                  "/nix/store/0g95h72qqdxlig31n6ahcz1ch1jsg9q4-rust-analyzer-unwrapped-2023-05-15/bin/rust-analyzer"
                ],
                "cwd": "/path/to/felis",
                "pid": 38646
              },
              {
                "cmdline": [
                  "/etc/profiles/per-user/gaborpihaj/bin/hx"
                ],
                "cwd": "/path/to/felis",
                "pid": 38411
              }
            ],
            "id": 1,
            "is_active": false,
            "is_focused": false,
            "is_self": false,
            "lines": 47,
            "pid": 863,
            "title": "hx"
          },
          {
            "cmdline": [
              "-zsh"
            ],
            "columns": 119,
            "cwd": "/path/to/felis",
            "env": {},
            "foreground_processes": [
              {
                "cmdline": [
                  "kitten",
                  "@",
                  "ls"
                ],
                "cwd": "/path/to/felis",
                "pid": 49915
              }
            ],
            "id": 2,
            "is_active": true,
            "is_focused": true,
            "is_self": true,
            "lines": 23,
            "pid": 972,
            "title": "kitty @ ls"
          },
          {
            "cmdline": [
              "-zsh"
            ],
            "columns": 119,
            "cwd": "/path/to/felis",
            "env": {},
            "foreground_processes": [
              {
                "cmdline": [
                  "-zsh"
                ],
                "cwd": "/path/to/felis",
                "pid": 983
              }
            ],
            "id": 3,
            "is_active": false,
            "is_focused": false,
            "is_self": false,
            "lines": 24,
            "pid": 983,
            "title": "/path/to/felis"
          }
        ]
      }
    ],
    "wm_class": "kitty",
    "wm_name": "kitty"
  }
]"#;
}
