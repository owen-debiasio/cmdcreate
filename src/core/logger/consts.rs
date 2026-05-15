use crate::utils::colors::COLORS;

#[derive(Copy, Clone)]
pub enum Severity {
    Normal = 0,
    Warn = 1,
}

impl Severity {
    pub(crate) fn to_colored_string(self) -> String {
        let (cyan, yellow, reset) = (COLORS.cyan, COLORS.yellow, COLORS.reset);
        match self {
            Self::Normal => format!("{cyan}LOG{reset}"),
            Self::Warn => format!("{yellow}WARN{reset}"),
        }
    }
}
