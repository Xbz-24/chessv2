#[cfg(test)]
mod tests {
    use super::*;

    // Create a mock SessionManager
    struct MockSessionManager {
        sessions: Vec<Session>,
    }

    impl MockSessionManager {
        fn new() -> Self {
            Self {
                sessions: Vec::new(),
            }
        }

        fn create_session(&mut self, session: Session) {
            self.sessions.push(session);
        }

        fn delete_session(&mut self, session_id: usize) -> Option<Session> {
            let pos = self.sessions.iter().position(|x| *x.id == session_id)?;
            Some(self.sessions.remove(pos))
        }

        fn get_session(&self, session_id: usize) -> Option<&Session> {
            self.sessions.iter().find(|&x| x.id == session_id)
        }
    }

    // Create a mock Session
    #[derive(Clone)]
    struct Session {
        id: usize,
        players: usize,
    }

    impl Session {
        fn new(id: usize) -> Self {
            Self {
                id,
                players: 0,
            }
        }

        fn add_player(&mut self) {
            self.players += 1;
        }

        fn remove_player(&mut self) {
            self.players -= 1;
        }
    }

    #[test]
    fn test_create_session() {
        let mut session_manager = MockSessionManager::new();
        let session = Session::new(1);
        
        session_manager.create_session(session);

        assert!(session_manager.get_session(1).is_some());
    }

    #[test]
    fn test_delete_session() {
        let mut session_manager = MockSessionManager::new();
        let session = Session::new(1);
        
        session_manager.create_session(session);

        assert!(session_manager.get_session(1).is_some());

        session_manager.delete_session(1);

        assert!(session_manager.get_session(1).is_none());
    }

    #[test]
    fn test_manage_players() {
        let mut session = Session::new(1);

        session.add_player();

        assert_eq!(session.players, 1);

        session.remove_player();

        assert_eq!(session.players, 0);
    }
}
