mod clap_man;
mod chat;
mod gui_manager;

use crate::clap_man as com;


fn main() {
    com::run_args(com::return_arg_array());
}