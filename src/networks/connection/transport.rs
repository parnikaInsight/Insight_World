use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::Boxed;
use libp2p::core::identity::Keypair;
use std::io::Result;
use libp2p::{development_transport, core, dns, identity, mplex, noise, tcp, websocket, yamux, PeerId, Transport};

pub async fn create_transport(local_key: Keypair) -> Result<Boxed<(PeerId, StreamMuxerBox)>> {
    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = development_transport(local_key).await?;
    Ok(transport)

    // let transport = {
    //     let dns_tcp = dns::DnsConfig::system(tcp::TcpTransport::new(
    //         tcp::GenTcpConfig::new().nodelay(true),
    //     ))
    //     .await?;
    //     let ws_dns_tcp = websocket::WsConfig::new(
    //         dns::DnsConfig::system(tcp::TcpTransport::new(
    //             tcp::GenTcpConfig::new().nodelay(true),
    //         ))
    //         .await?,
    //     );
    //     dns_tcp.or_transport(ws_dns_tcp)
    // };

    // let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
    //     .into_authentic(&local_key)
    //     .expect("Signing libp2p-noise static DH keypair failed.");

    // Ok(transport
    //     .upgrade(core::upgrade::Version::V1)
    //     .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
    //     .multiplex(core::upgrade::SelectUpgrade::new(
    //         yamux::YamuxConfig::default(),
    //         mplex::MplexConfig::default(),
    //     ))
    //     .timeout(std::time::Duration::from_secs(20))
    //     .boxed())
}