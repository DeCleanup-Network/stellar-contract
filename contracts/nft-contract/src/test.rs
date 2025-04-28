#![cfg(test)]
use crate::{NFTContract, errors::NFTError};
use soroban_sdk::{testutils::{Address as _}, vec, Address, Env, String, Vec};

fn setup_contract() -> (Env, Address) {
    let env = Env::default();
    let contract_id = env.register(NFTContract, ());
    (env, contract_id)
}

fn create_test_address(env: &Env) -> Address {
    Address::generate(&env)
}

#[test]
fn test_nft_minting() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let user = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    // Testing minting
    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_all_auths();

    let result = env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            user.clone(),
            name.clone(),
            description.clone(),
            image_url.clone(),
            level
        )
    });

    assert!(result.is_ok());

    let nft_token_detail = env.as_contract(&contract_id, || {
        NFTContract::get_metadata(env.clone(), 0)
    });

    assert_eq!(nft_token_detail.owner, user);
    assert_eq!(nft_token_detail.name, name);
    assert_eq!(nft_token_detail.description, description);
}


#[test]

#[should_panic(expected = "Unauthorized function call for address")]
fn test_admin_access_control() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let user = create_test_address(&env);


    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_auths(&[]);
    let _result = env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            user.clone(),
            name.clone(),
            description.clone(),
            image_url.clone(),
            level
        )
    });

}


#[test]
fn test_transfer_ownership() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let owner = create_test_address(&env);
    let nft_reciever = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_all_auths();
    let _ = env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            owner.clone(),
            name.clone(),
            description.clone(),
            image_url.clone(),
            level
        )
    }).unwrap();


    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        NFTContract::transfer(
            env.clone(),
            owner.clone(),
            nft_reciever.clone(),
            0,
        )
    });
    assert!(result.is_ok());
}


#[test]
fn test_transfer_ownership_with_wrong_tokenid_should_fail() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let owner = create_test_address(&env);
    let nft_reciever = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_all_auths();
    let _ = env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            owner.clone(),
            name.clone(),
            description.clone(),
            image_url.clone(),
            level
        )
    }).unwrap();


    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        NFTContract::transfer(
            env.clone(),
            owner.clone(),
            nft_reciever.clone(),
            1,
        )
    });
    assert!(result.is_err());
}


#[test]
fn test_transfer_ownership_for_non_owner_should_fail() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let owner = create_test_address(&env);
    let nft_reciever = create_test_address(&env);
    let nft_non_owner = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_all_auths();
    let _ = env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            owner.clone(),
            name.clone(),
            description.clone(),
            image_url.clone(),
            level
        )
    }).unwrap();


    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        NFTContract::transfer(
            env.clone(),
            nft_non_owner.clone(),
            nft_reciever.clone(),
            0,
        )
    });
    assert!(result.is_err());
}


#[test]
fn test_burn_success() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let user = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            user.clone(),
            name,
            description,
            image_url,
            level,
        )
    }).unwrap();

    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        NFTContract::burn(env.clone(), user.clone(), 0)
    });

    assert!(result.is_ok());
}


#[test]
fn test_burn_non_existing_token_should_fail() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let user = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        NFTContract::burn(env.clone(), user.clone(), 99) // token ID 99 does not exist
    });

    assert!(result.is_err());
}


#[test]
fn test_burn_token_by_non_owner_should_fail() {
    let (env, contract_id) = setup_contract();
    let admin = create_test_address(&env);
    let real_owner = create_test_address(&env);
    let attacker = create_test_address(&env);

    env.as_contract(&contract_id, || {
        NFTContract::init_admin(env.clone(), admin.clone())
    }).unwrap();

    let name = String::from_str(&env, "NFT Token");
    let description = String::from_str(&env, "NFT Description");
    let image_url = String::from_str(&env, "https://pinata.com");
    let level: u32 = 7;

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        NFTContract::mint(
            env.clone(),
            real_owner.clone(),
            name,
            description,
            image_url,
            level,
        )
    }).unwrap();

    env.mock_all_auths();
    let result = env.as_contract(&contract_id, || {
        NFTContract::burn(env.clone(), attacker.clone(), 0)
    });

    assert!(result.is_err());
}