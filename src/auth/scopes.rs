// Copyright 2020 Nick Samson -- See LICENSE for copyright info.

//! Contains types and functions related to interacting with FimFic auth scopes.


use std::str::FromStr;
use std::error::Error;

/// This enum contains all of the scopes available through the FimFic OAuth API.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Scope {
    /// Allows an app to post blog posts
    WriteBlogPosts,
    /// Allows an app to read non-public blog posts
    ReadBookshelves,
    /// Allows an app to write items to create/edit bookshelves
    WriteBookshelves,
    /// Allows an app to read items from a private bookshelf
    ReadBookshelfItems,
    /// Allows an app to add/remove items to a bookshelf
    WriteBookshelfItems,
    /// Allows an app to read the PMs of a user
    ReadPms,
    /// Allows an app to write the PMs of a user
    WritePms,
    /// Allows an app to follow/unfollow users
    WriteFollowers,
    /// Allows an app to read unpublished chapters/stories
    ReadStories,
    /// Allows an app to write/edit/publish/delete stories
    WriteStories,
    /// Allows an app to write/edit/delete comments
    WriteComments,
    /// Allows an app to read private account information
    ReadUser,
    /// Allows an app to modify account information
    WriteUser,
    /// Allows an app to see what chapters a user has read
    ReadChapterRead,
    /// Allows an app to mark chapters as read/unread
    WriteChapterRead
}

impl Scope {
    /// Returns a string which represents the scope name FimFic recognizes
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::WriteBlogPosts => "write_blog_posts",
            Scope::ReadBookshelves => "read_bookshelves",
            Scope::WriteBookshelves => "write_bookshelves",
            Scope::ReadBookshelfItems => "read_bookshelf_items",
            Scope::WriteBookshelfItems => "write_bookshelf_items",
            Scope::ReadPms => "read_pms",
            Scope::WritePms => "write_pms",
            Scope::WriteFollowers => "write_followers",
            Scope::ReadStories => "read_followers",
            Scope::WriteStories => "write_stories",
            Scope::WriteComments => "write_comments",
            Scope::ReadUser => "read_user",
            Scope::WriteUser => "write_user",
            Scope::ReadChapterRead => "read_chapter_read",
            Scope::WriteChapterRead => "write_chapter_read",
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Contains a string that failed to parse into a [Scope]
#[derive(Debug, Clone)]
pub struct ParseScopeError(String);

impl FromStr for Scope {
    type Err = ParseScopeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "write_blog_posts" => Ok(Scope::WriteBlogPosts),
            "read_bookshelves" => Ok(Scope::ReadBookshelves),
            "write_bookshelves" => Ok(Scope::WriteBookshelves),
            "read_bookshelf_items" => Ok(Scope::ReadBookshelfItems),
            "write_bookshelf_items" => Ok(Scope::WriteBookshelfItems),
            "read_pms" => Ok(Scope::ReadPms),
            "write_pms" => Ok(Scope::WritePms),
            "write_followers" => Ok(Scope::WriteFollowers),
            "read_followers" => Ok(Scope::ReadStories),
            "write_stories" => Ok(Scope::WriteStories),
            "write_comments" => Ok(Scope::WriteComments),
            "read_user" => Ok(Scope::ReadUser),
            "write_user" => Ok(Scope::WriteUser),
            "read_chapter_read" => Ok(Scope::ReadChapterRead),
            "write_chapter_read" => Ok(Scope::WriteChapterRead),
            _ => Err(ParseScopeError(s.to_string()))
        }
    }
}

impl Error for ParseScopeError {}
impl std::fmt::Display for ParseScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse {} as a FimFic API scope", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_parse() {
        let r = Scope::from_str("write_chapter_read").unwrap();
        assert_eq!(r, Scope::WriteChapterRead);
        let _ = Scope::from_str("Gibberish").unwrap_err();
    }
}
