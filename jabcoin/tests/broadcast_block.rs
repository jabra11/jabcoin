mod common;

#[test]
fn broadcast_block()
{
    // setup mock nodes
    let mut nodes = vec![];
    for i in 0..10
    {
        let j = i as u8;
        nodes.push(common::setup_mock_node(i, Some((j, j, j, j)), None));
    }

    for i in 0..nodes.len()
    {
        for j in 0..nodes.len()
        {
            if i == j
            {
                continue;
            }
            let rhs = nodes[j].clone();
            nodes[i].connect_node(rhs);
        }
    }

    for node in &nodes
    {
        println!("{}", serde_json::to_string(node).unwrap());
    }

    todo!();
}
