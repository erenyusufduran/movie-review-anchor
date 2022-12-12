use anchor_lang::prelude::*;

declare_id!("6AFxYYmtTLLd6BbLu6DyCo3pZPANkqQkkLZTTe5hR5E");

#[program]
pub mod movie_review {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!(
            "Title: {}, Description: {}, Rating: {}",
            title,
            description,
            rating
        );

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!(
            "Title: {}, Description: {}, Rating: {}",
            title,
            description,
            rating
        );

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.description = description;
        movie_review.rating = rating;

        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(init, seeds = [title.as_bytes(), initializer.key().as_ref()], bump, payer = initializer, space = 8 + 32 + 1 + 4 + title.len() + 4 + description.len())]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateMovieReview<'info> {
    #[account(mut, seeds = [title.as_bytes(), initializer.key().as_ref()], bump, realloc = 8 + 32 + 1 + 4 + title.len() + 4 + description.len(), realloc::payer = initializer, realloc::zero = true)]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = reviewer, has_one = reviewer)]
    movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    reviewer: Signer<'info>,
}

#[account]
pub struct MovieAccountState {
    pub reviewer: Pubkey, // 32
    pub rating: u8,       // 1
    pub title: String,    // 4 + len()
    pub description: String,
}
