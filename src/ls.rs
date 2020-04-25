mod discover;

use discover::discover;

fn main()
{
    let hosts = match discover()
    {
        Ok(res) => res,
        Err(error) => panic!("We ran into a problem: {}", error),
    };

    hosts
        .iter()
        .for_each(|host| println!("{}@{}", host.user, host.ip))
}
