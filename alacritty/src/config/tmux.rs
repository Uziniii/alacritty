use alacritty_config_derive::ConfigDeserialize;

#[derive(ConfigDeserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Tmux {
    pub enabled: bool,
}
