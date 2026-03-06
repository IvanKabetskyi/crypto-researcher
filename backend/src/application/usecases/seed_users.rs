use crate::domain::user::entities::User;
use crate::infrastructure::repositories::user::UserRepository;

struct SeedUser {
    email: &'static str,
    password: &'static str,
}

const SEED_USERS: &[SeedUser] = &[
    SeedUser {
        email: "ivankabeckii@gmail.com",
        password: "CryptoRes2026!",
    },
    SeedUser {
        email: "ikapustin@icloud.com",
        password: "CryptoRes2026!",
    },
    SeedUser {
        email: "krivonosroman1@gmail.com",
        password: "CryptoRes2026!",
    },
];

pub async fn seed_users() {
    let user_repo = UserRepository::new().await;

    for seed in SEED_USERS {
        match user_repo.find_by_email(seed.email).await {
            Ok(Some(_)) => {
                tracing::info!("User {} already exists, skipping", seed.email);
            }
            Ok(None) => {
                let hash = bcrypt::hash(seed.password, 10).expect("failed to hash password");
                let user = User::new(seed.email, &hash, None);
                match user_repo.create_user(&user).await {
                    Ok(_) => tracing::info!("Created user: {}", seed.email),
                    Err(e) => tracing::error!("Failed to create user {}: {}", seed.email, e.message),
                }
            }
            Err(e) => {
                tracing::error!("Failed to check user {}: {}", seed.email, e.message);
            }
        }
    }
}
