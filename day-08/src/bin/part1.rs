use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    start_node: String,
    left_node: String,
    right_node: String,
}

fn get_node_pos(nodes: &Vec<Node>, node: String) -> usize {
    let mut pos: usize = 0;
    for i in 0..nodes.len() {
        if nodes[i].start_node == node {
            pos = i;
        }
    }

    pos
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("input1.txt");
    let mut instructions = "".to_string();
    let mut node_strings: Vec<String> = vec![];
    let mut nodes: Vec<Node> = vec![];

    let mut reading_nodes = false;
    for line in lines {
        if line == "" {
            reading_nodes = true;
            continue;
        }

        if reading_nodes {
            node_strings.push(line);
        } else {
            instructions += &line;
        }
    }
    for s in node_strings {
        let mut n: Node = Node {
            start_node: "".to_string(),
            left_node: "".to_string(),
            right_node: "".to_string(),
        };

        let split1 = s.trim().split("=").collect::<Vec<&str>>();
        n.start_node = split1[0].replace(" ", "").to_string();
        let lr_nodes_str = split1[1].replace(" ", "").replace("(", "").replace(")", "");
        let lr_nodes = lr_nodes_str.split(",").collect::<Vec<&str>>();
        n.left_node = lr_nodes[0].to_string();
        n.right_node = lr_nodes[1].to_string();

        nodes.push(n);
    }

    let chars: Vec<char> = instructions.chars().collect();
    let mut i = 0;
    let mut stepcount = 0;

    let mut current_node = get_node_pos(&nodes, "AAA".to_string());
    loop {
        println!("debug: i: {}, stepcount: {}", i, stepcount);
        if i >= chars.len() {
            i = 0;
        }
        if &nodes[current_node].start_node == &"ZZZ".to_string() {
            break;
        }
        let jumpnode: &str;
        if chars[i] == 'R' {
            jumpnode = &nodes[current_node].right_node;
        } else if chars[i] == 'L' {
            jumpnode = &nodes[current_node].left_node;
        } else {
            panic!("chars[{}] is {}", i, chars[i]);
        }
        current_node = get_node_pos(&nodes, jumpnode.to_string());

        stepcount += 1;
        i += 1;
    }

    println!("i: {}, stepcount: {}", i, stepcount);
}
