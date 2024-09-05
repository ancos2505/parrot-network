use h10::http::{
    headers::{Authorization, IntoHeader, UserAgent, WWWAuthenticate},
    method::Method,
    request::Request,
    status_code::StatusCode,
};

use crate::{
    node::server::{
        result::{ServerError, ServerResult},
        ServerResponse,
    },
    proto::{
        blockchain::wallet::PublicKey,
        node_session::{
            fields::Realm,
            pki_client_challenge::PkiClientChallenge,
            pki_server_challenge::{fields::ServerChallenge, PkiServerChallenge},
        },
    },
    NODE_CONFIG,
};

#[derive(Debug)]
pub(super) struct NewPeer;

impl NewPeer {
    const REALM: Realm = Realm::ParrotNode;

    pub(super) fn handler(request: &Request) -> ServerResult<ServerResponse> {
        if *request.method() == Method::Get {
            let req_user_agent = match request
                .headers()
                .get(UserAgent::default().into_header().name())
            {
                Some(header_entry) => header_entry,
                None => return Ok(ServerResponse::new(StatusCode::BadRequest)),
            };

            let req_authorization = match request
                .headers()
                .get(Authorization::default().into_header().name())
            {
                Some(entry) => entry,
                None => return Ok(ServerResponse::new(StatusCode::BadRequest)),
            };

            // * Check if client's client_challenge is valid - BEGIN

            let req_pki_client_challenge =
                req_authorization.value().parse::<PkiClientChallenge>()?;

            let client_realm = req_pki_client_challenge.realm();
            let client_challenge = req_pki_client_challenge.challenge();
            let client_signature = req_pki_client_challenge.signature();
            let client_public_key = req_pki_client_challenge.public_key();

            // TODO: Implement more sophisticated validation
            if *client_realm != Self::REALM
                || !client_challenge.verify(client_signature, client_public_key)
            {
                return Ok(ServerResponse::new(StatusCode::BadRequest));
            } else {

                // TODO: Add to an in-memory Database for clients handshaking process
            }
            // * Check if client's client_challenge is valid  - END

            println!("NodeServer: (NewPeer): [{}]", req_user_agent);

            let server_secret_key = NODE_CONFIG
                .get()
                .and_then(|config| config.secret_key())
                .ok_or(ServerError::NodeSigningKey(
                    "Error on getting signingkey".into(),
                ))?;

            let server_public_key = PublicKey::from(server_secret_key);

            let server_challenge = ServerChallenge::new(&req_pki_client_challenge);

            let pki_server_challenge = {
                let signature = server_secret_key.sign(&server_challenge.as_bytes());
                PkiServerChallenge::builder()
                    .realm(Self::REALM)
                    .challenge(server_challenge)
                    .signature(signature)
                    .public_key(server_public_key)
                    .finish()
            };

            let www_authenticate = WWWAuthenticate::new(&pki_server_challenge.to_string())?;

            Ok(ServerResponse::new(StatusCode::Unauthorized).add_header(www_authenticate))
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        }
    }
}

#[cfg(test)]
mod tests {
    use h10::http::headers::{Authorization, IntoHeader};

    use crate::proto::{
        blockchain::result::BlockchainProtoResult,
        node_session::pki_client_challenge::PkiClientChallenge,
    };

    #[test]
    fn parse_autorization() -> BlockchainProtoResult<()> {
        let input = r#"Authorization: PKI realm="0001", client_challenge="ab316ca0170234ea", signature="5ab5587ed9c5b13934507f5db88fa3c83cabd7f25ac2e9d21a98715270b8b50cfb04dc7c2c6a86469a8afeffd93d34a13b769e2452339b154ad4d715d6def4e2cda71babfeded293022acac29cd344ecba3cbc099ca55cb63e0982bba038fa1fcfef404b321553f2a0a373b51cb33e1d943126bec38ef68e92f2c94b0bf96f8e24e4beeedd7291a8343eacaff6d31951bb43a2c802f3f23b5c4c935735514b0dde0734df66939206a02a53ec15c6b55045587e1ec574dd43f0b5b8391c4764dd4868d0e8441e631c669f7b6379f04b5ca70e647259e6696ad7e27a3c08e7da54d3262dcb259c28922fb1f3586237e85a14abd02694846577220f3d178e9d30bda6fe01f9bfaaeed6cdf9ca4eaa91f354cb638c19ef3ee9d32d04d839b3ab5ba870690bb43ae0b96537519e4a14a1e6e249bd6332a0c6db451515ffa5fb643c4069092bac98a0f0d350b146e2a86ca4c4fc1d16354f327aaf365d19cea00ec5271dab842ddd82a5679ca7a7b9888e31f1a18298ec7a9f3b47cf4634670a81e2cd44110e1e8f9c9956fb5469c6636507a7b8f42dd912c757be6e9cc5bec3285a2623cd445d6ee63d4a619a5600f74c254b437b61546cb8a3e50baec552a4053c8138d819a67ec727b5e42f2a6c04872dda7871dfcbb41b7b3282b42fb4bcec8e3f9c6e4c59c853846f6dc83493f85eb736fd409b89bc344929e89217afa87221fdd5dd0f43120a0253cc99c96027074c8253dbe6d346edad8661843d5aeaea4dc7dcd39b7836d8d270c8853c92eb9f059498f291669e26b8d63dae3d6b388377d27f293621798bf6bd006294520bb8949a76b3b36d97290a7f0ce96d9238a31e68c8018d36d1045ce8c71a9a24b5903d7447b1efb0bcd1fcf98b231a951e5301af542566d09bc1eeecdea7cdac7cfbee790c910844a3148337ec2169b72f327eaa44f5e355b7e548ea34e60cbe542509dbef4d36257e1109e12f8edbbb187a93133f9833f2fdfaf52867a51149737dde485dcc2d7a5078b121ca41b3c9719874acb178249a5c2b4cdf7d81f55ab1283c9d089d46a04f26db12d6a1d3a1bc29714ddbc1a6b0c14940b7fd93decc07091793d1281344b7349a23e79b22f3a57353b1886962bc4250da7f702d4bc1984a09b0f24cebd5c863c2dc2499e248bfc1ab3cd48294f332b938edd31d1662f3f72ade0def4993355a03b621d59c0df1bb218c873b64ce5cdde2009eb35883b460767bb5d60115f9df893ed2124264c5a605c3ef416b2bfe6d471a6a034d74beca99cf457d366c1688b1896cfbaf1cdb096e5aba483dde25acf63b4c4bc9614bbb90b47be6e0d224ebbdaa36cee20e57696f5430dd4d9b1b0d1bfc3cd6a6874dab8f839b56d922e503d1bbfe6cd8193bc3f410393b76c41289c0d0b5e7af45e244344d3300ae25f0b55a56c35ace408a6c20aecab937f1e3dbb8ce2b68817b7b9e4442d09ef6539b8f08987dd0d8f24b97db90330889afd045dd6ac0f8bf1cb36eba4151ad1c26ddf3e9fdbbcc6c77d38e6c4d96952993690e5100daf562b6f87a739c47372d83aa920e6c0e704a0f327ac89ae4f13651794d9f1576665b02dbc0a167622d81bfbc452453f7967a75985295d49d7f0897cda955e744fb38c8cb22e96b24d5ab6c5a4eaee8ad8a5a9eb8acea6248620fc7bf10d5b67bf4cdb1d7327a7e7792629ab6c24713d514385686f754a16de75a3651b7313892a5358b22f9a57bbe675b8620ad665437d69cc30f3ce6710d9be0f1b09df70d4d4570699ee9c2cb6b196442aebced335000000000000000", public_key="0a69d8aa91c8e0834744593ba062b7046e54a4b60757bbf4136007260090cadd6ae7c7a902d5fde3f82049344633c9cd6b4d0ba91af37d6ccc9915fe217d46d13fa81182e6cd2256d4e3968da1182011a7b0e6ada0521969312841308c94096b50e3045021bcb2ae1ca62e423ac9504b6631f81664151808708d08a614c44f7d9435d1ddeb080fe407e6a56e7c49723b45a68e061e1887841a141e5954eb521c6870b178944aea2c53a045658983cf6468240b868c03dd9b583992226169a870a3444dcae157e95a29199dcab64911c9b14d5216563cb3cf1a8c2ae0b168b485e48f6863f7ce9a1f1322018aabd386aaf95f73d743e7684ea6757f58961e0841d15b4cea4a305169d76d68174e3c4a491da0476175917d148b4075d394f1dfeb8f2453d28de5862b4d23babee33c3e588ff46b80aba4012cc278d32fa854e0e9b5994c73245b640b1a566e11a75a304637b510eb1852528a64407188abd9129dfc34d7da43dca35d40a302d80f4018e3b28fd2ee315e66514c40bd36d28596675c151d8c7e1409491e600827cb91aec63e64efda61d14c3a04d071e5eec91ff2307203a86aaedddd7989093595ecff15e942bf143949136127602279a6d1c02913732cb1e8ff9cb347def474334a54847a1619fc53d61d8c60cb0483175e8a6e2803181424c6b9f67e80b72ed8b0e76361c297583fc7af46d434ea5d66e1e12d86d408253a0f39aebabab7a426503960b2417a98a3574464f8de2ed9523ea21157cb5032507712c2a59a77624c0a9053be9e8c6d292aa2f72624817555c41d038eea3a4a06634790943347ef756b5c82a8384f5c230635a033e5b639494bf36663e6ac5032b880569c1f220aa4566d6468a2367ab9974067966c67ab46d278162ff51b91bab9c59f22c187f4162925d3afc33d918e4d984b1048b1e27938f98d3f9fbe2816d2d189ad0d9c2307129362c14890a6c985280df9abef12d28392c63662df94768ef5e35e64e567a8ba66025bc72d705635f0293554eef4c9cc153751161208459854a48c4982852e5a7064ad838c88b0b51243e88e19998e7a67a756eb2a3910a5849f6ada28a65919a6548e6f08a849f2dd9576b672424c4f5855a13f11374d3ca51ac98a1bdf62ac523a5346a08f72eac6a00d6c581ea254fa14965a6ed893e6f86790942d70a2cf58a503141b30f788be4998152a4c414596884991559a6932463ebab5803c8dba8e00a2157544a80a6b7ecb868ebc7db22c2b43912c02cb1707e714edaa7a60c58b9a09ace83458c7a2f406f9031a4ebe30785a54114d09b949142808f10779c2b7d85a03804c7030cee3771c0e9ee81fc70c4571ce3bd49fc3164d42654803d04e75101b612904bf3224e37098c04d8cbcabbbd9185ad922459b2c524b2eb8863ead85573aac39a309a0b5be218a824ba2b85a63fb7deef194661c672f21c49a541b91cd043e55e209c991985708e8fe857d00d12e39eef35deb61441e234471a69d4a965931d16984129930273dcf5109e637167dafd1108d77b223278999eee8291c8fdbbd5ee7965f0fd03d718539081c8811563695d21c1f824edc314b9be87759b478a122cd7e42d8623bae1380d2fbf470cfcc6abd5ed96acd4c0c846720ac2d2a25aadbb4b64f766afebb27a743cbae591e1731a5390601307781ca1b62047a147c87486a0dba671a148ca7585bd3658ec3a7025551b5e9060075d2d08a2f0d5705d9d70a20f4a22914c5fb5590bcccd37018efeca31535c40829dc940045e517c9849ccce59581532d34fd3fc1d0578f9130b546a2a5f910d958801f930f21ac17f86bdd87a9ea8110d9261c1b6e46ab43469c6875ecd03dc4363b31110755e71b3c715952554055801a7bf22155530d8c119474da92944c56c698a133110e196a2d25ff729ca81495412396fa12a6f49a2438f69519316174540451aadb9dae803cbb1b058c6b8c3cf8540ec07b9fecf1ca832966965e15aae00f1389228e287ec7c9085cf728a8417d6184907a70946786ea1c984da52555fa47b8d2c6801595fbf1a51b147905b3d3d051423c85c2c260dbd04c1959c7318d1758327587c190444e214610c8986869e15719a4945c212eb4f6ee2165120528a3ba6c7b6ae57848fe6e8cb5c29116991f0a47a91d91bde620a8b6b23324cd14ae1ea9227b025380e489aaa5ef805c3668f0e4a9b22566cf47cee1ce64303113b0514c6d24231a19ce37300552ca7ca7db90dc8556430fc61f9ded7308c715afc91c34b6c5ae7572dc355de766712097a30f0367a7bcd8ea1b270a3cf9e7e10d4a3359486628bce3c60a58744186ad05670a60f04945c38e7a2d4baa070a2dad2baae0043e79952733c7868e05f2bde9b65954b1cd04087e5141b662738b7dcaad46b17f388dac691d010c958140e2696803622ea514887829b7bd99335591efe7c958a38bad3aa68852df176a30a719da61a064f89a7b2909b4a669dabc69334135f846536e7f1036aa416a6e14b7c93e2389d50b""#;
        let autorization: Authorization = input.parse()?;
        let res = (&**(autorization.into_header()).value()).parse::<PkiClientChallenge>();
        assert!(res.is_ok());
        Ok(())
    }
}
