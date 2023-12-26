fn main() {
    let user = User::new(String::from("Winike"), 14);
    println!("O nome e {}", user.get_name());
    println!("A idade e {}", user.get_age());

    let post = Post::new("Teste", "Post");

    user.make_post(post);

}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

struct Post {
    title: String,
    description: String,   
}


impl User {
    fn new(name: String, age: i32) -> User {
        User {
            id: 12,
            name,
            age
        }
    }

    fn get_name(&self) -> &str {
        &self.name                
    }

    fn get_age(&self) -> i32 {
        self.age
    }

    fn make_post(&self, post: Post) -> () {
        println!("The user {} make the post with title {}", self.get_name(), post.title);
    }
}

impl Post {
    fn new(title: &str, description: &str) -> Post {
        Post { title: title.to_string(), description: description.to_string() }
    }
}

