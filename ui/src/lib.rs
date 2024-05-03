pub fn clear_terminal() {
   let term = console::Term::stderr();
   term.clear_screen().expect("Failed to clear terminal");
}