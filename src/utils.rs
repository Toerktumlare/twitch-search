use crate::api::streams::Entry;

pub fn print(entry: &Entry) {
    print!("{} | ", entry.language);
    print!("https://twitch.tv/{:<16} | ", entry.user_name);
    print!("{:>1} viewers | ", entry.viewer_count);
    print!("{} | ", entry.started_at);
    print!("{}\n", entry.title);
}
