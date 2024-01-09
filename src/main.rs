use curl::easy::{Easy, self};
mod access_token;

fn main() {
    access_token::print_access_token();
}
