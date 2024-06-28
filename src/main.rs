use ldk_node::Builder;
use ldk_node::lightning_invoice::Bolt11Invoice;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use std::str::FromStr;
use std::io::{self, Write};

fn setup_node() -> ldk_node::Node {
    let mut builder = Builder::new();
    builder.set_network(Network::Regtest);
	builder.set_esplora_server("http://localhost:30000".to_string());
    //builder.set_esplora_server("https://blockstream.info/testnet/api".to_string());
    //builder.set_gossip_source_rgs("https://rapidsync.lightningdevkit.org/testnet/snapshot".to_string());
    builder.build().unwrap()
}

fn start_node(node: &ldk_node::Node) {
    node.start().unwrap();
}


fn perform_operations(node: &ldk_node::Node) {
    loop {
        println!("Enter command (new_address, sync, list_balance, connect, send_payment, stop):");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match command.trim() {
            "new_address" => {
                let address = node.onchain_payment().new_address();
                println!("New address: {:?}", address);
            },
            "list_balance" => {
				let spendable_balance = node.list_balances().spendable_onchain_balance_sats;
				let total_balance = node.list_balances().total_onchain_balance_sats;
                println!("Spendable Balance: {:?}", spendable_balance);
                println!("Total Balance: {:?}", total_balance);
            },
            "sync" => {
                node.sync_wallets().unwrap();
                println!("Synced...");
            },
            "connect" => {
                println!("Enter node ID:");
                let mut node_id_str = String::new();
                io::stdin().read_line(&mut node_id_str).unwrap();
                let node_id = PublicKey::from_str(node_id_str.trim()).unwrap();

                println!("Enter node address (IP:PORT):");
                let mut node_addr_str = String::new();
                io::stdin().read_line(&mut node_addr_str).unwrap();
                let node_addr = SocketAddress::from_str(node_addr_str.trim()).unwrap();

                node.connect_open_channel(node_id, node_addr, 10000, None, None, false).unwrap();
                println!("Connected to node and channel opened.");
            },
            "send_payment" => {
                println!("Enter invoice:");
                let mut invoice_str = String::new();
                io::stdin().read_line(&mut invoice_str).unwrap();
                let invoice = Bolt11Invoice::from_str(invoice_str.trim()).unwrap();
                node.bolt11_payment().send(&invoice).unwrap();
                println!("Payment sent.");
            },
            "stop" => {
                node.stop().unwrap();
                println!("Node stopped.");
                break;
            },
            _ => println!("Unknown command."),
        }
    }
}

fn main() {
    let node = setup_node();
    start_node(&node);
    perform_operations(&node);
}