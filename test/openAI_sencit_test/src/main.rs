mod clap_man;
mod chat;
mod db_man;
mod gui_view;
mod sttttts;

use crate::clap_man as com;



fn main() {
    com::run_args(com::return_arg_array());
}