use crate::replay::ReplayCommand;
use std::convert::TryInto;

impl TryInto<redis::Cmd> for ReplayCommand {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<redis::Cmd, Self::Error> {
        let mut cmd = redis::cmd(self.command());

        self.args().iter().for_each(|a| {
            cmd.arg(a);
        });

        Ok(cmd)
    }
}
