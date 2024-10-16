let mut poll = Poll::new()?;
poll.registry().register(<event>, <token>, <interest> )?;

