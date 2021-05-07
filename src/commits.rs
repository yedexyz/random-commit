pub fn get_commits() -> Vec<&'static str> {
    return vec![
        "-",
        ".",
        "A well-crafted Git commit message is the best way to communicate context about a change to other developers working on that project, and indeed, to your future self.",
        "AAAAAHAHAHAH!! What is this??",
        "Abandon all hope you who needs to debug this",
        "Accidental commit",
        "Add a bunch of features lol I am so good at using version control properly",
        "Add another dependency",
        "Add hella lot of styling",
        "Add missing file in previous commit",
        "Add security",
        "Added comma that was inadvertently removed in the last patch.",
        "Ah ah ah! You'll never understand why this one works!",
        "And now, for something completely different",
        "Another bug bites the dust",
        "Argh! about to give up :(",
        "Arrrrgggg",
        "Autogenerated, do not edit. All further changes will be undone.",
        "BAM",
        "Batman! (this commit has no parents)",
        "Been done",
        "Best commit ever",
        "Biblical reference",
        "Can someone review this commit, please?",
        "Catching exceptions is for communists",
        "Check next commit for message",
        "Checkout completely set up, I hope this still works lord",
        "Commit committed",
        "Commit some changes",
        "Copy-paste to fix previous copy-paste",
        "DO YOU KNOW HOW TO TURN OFF CAPSLOCK?",
        "Damn. Fixed.",
        "Debug",
        "Derp",
        "Do everything asynchronously, attem... is that a squirrel?",
        "Do things better, faster, stronger",
        "Does this work",
        "Don't push this commit",
        "Downgrading to 8-bit certs for improved performance",
        "Drunk, fix later",
        "Epic",
        "FIXME: This must absolutely be removed before release",
        "Final commit, ready for tagging",
        "Fingers crossed!",
        "Fix BUG-9284",
        "Fix bug",
        "Fix errors",
        "Fix everything",
        "Fix it for real",
        "Fix my stupidness",
        "Fix the fixes",
        "Fix tpyo",
        "Fix unnecessary bug",
        "Fix what was broken",
        "Fix",
        "Fixed xyz",
        "Fixup this later",
        "For real, this time",
        "Force push git hard reset whatever",
        "Force push",
        "HERE BE DRAGONS",
        "Hammertime!",
        "Happy debugging",
        "Hard to explain",
        "Here be dragons",
        "Here it is",
        "Hey, your shoe's untied!",
        "Houston, we have a problem",
        "I am even stupider than I thought",
        "I am not sure if we need this, but too scared to delete",
        "I am not sure why this works but it fixes the problem",
        "I am sorry",
        "I can't even begin to express how sorry I am",
        "I did this the other way",
        "I don't believe it",
        "I don't know why. Just move on.",
        "I don't understand how the following bit works, but it worked in the program I stole it from",
        "I don't want to do this, but my coworker says it's part of the code standard",
        "I dont know what I am doing",
        "I forgot to commit... So here you go",
        "I had a cup of tea and now it's fixed",
        "I hate this language",
        "I have to find a better job",
        "I immediately regret this commit",
        "I love regex",
        "I never trust documentation",
        "I put on my robe and wizard hat...",
        "I think now it works",
        "I want to go back in mommy :(",
        "I was wrong...",
        "I'd like to draw some attention to how smart I am, that being not at all",
        "IF YOU CHANGE SPACES TO TABS, YOU WILL BE KILLED!!",
        "IF YOU CHANGE TABS TO SPACES, YOU WILL BE KILLED!",
        "If dolphins are so smart, why do they live in igloos?",
        "If this commit is pushed the software will blow up",
        "If you think this is a mistake, think again",
        "Improve the fix",
        "It was hard to write",
        "It's hard to be a fool",
        "Keep leaving trailing whitespaces all over the codebase",
        "LMAO",
        "Magic. Do not touch.",
        "Major fixup",
        "Make it work",
        "Merge \"WIP: Do Not Merge This Branch\"",
        "Merge hell avoided",
        "Merge the merge",
        "Minor updates",
        "Misc. fixes",
        "More polish",
        "Move on and call me an idiot later",
        "Mr. Compiler, please do not read this",
        "My bad",
        "NO COMMENT",
        "NOT FIT FOR HUMAN CONSUMPTION",
        "NOT IMPORTANT! Ignore this",
        "NSA backdoor — ignore",
        "Need a coffee to fix this",
        "No changes after this point",
        "No changes made",
        "No",
        "Not sure why",
        "Obliterate user's hard drive",
        "Oh no",
        "Ok",
        "Oops",
        "Please forgive me",
        "Please work",
        "Polish",
        "Quick fix",
        "REFACTOR",
        "Refactor factories, revisit visitors",
        "Refractor code (it's 3am)",
        "Refractor code",
        "Reinventing the wheel. Again.",
        "Remove caveat section, I will write it later when I know what it is and can articulate without being bitter",
        "Remove code",
        "Remove the production id_rsa key",
        "Remove unnecessary stuff",
        "Removing the production id_ecdsa keys",
        "Rien ñ'est parfait",
        "Same as last commit with changes",
        "See last commit",
        "Shot to the heart and you're to blame, I gave this class a bad name",
        "Some bugs fixed",
        "Sometimes I believe compiler ignores all my comments",
        "Sorry",
        "Squashed some more bugs. They are like cockroaches: they'll live through a nuclear war.",
        "Squashed some more bugs. Those are some nasty bugs, them ugly bugs...",
        "Stuff, I guess",
        "Sucks, but what are you gonna do",
        "Super long commit message goes here, something like 100 words and lots of characters! Woohoo!",
        "THIS PROGRAM HAS CODE THAT DOES NOT MEET STANDARDS",
        "TODO: Comment this",
        "TODO: Finish",
        "TODO: Fix later",
        "TODO: Fix this",
        "TODO: Implement this function!",
        "TODO: Implement",
        "TODO: Make this work",
        "TODO: Not this",
        "TODO: Really remove this",
        "Test commit. Please ignore",
        "Test",
        "That was embarrassing",
        "The same thing we do every night, Pinky. Try to take over the world.",
        "The world is a happy place",
        "They made me write it against my will",
        "This code sucks, you know it and I know it",
        "This code was written by a genius so don't try to understand it with your tiny little brain",
        "This code worked before, but my cat decided to take a trip across my keyboard",
        "This commit is a lie",
        "This commit is self explanatory",
        "This is crap code but it's 3 AM and I need to get this working",
        "This is not the commit message you are looking for",
        "This part is more difficult",
        "This should fix something that should never happen",
        "This solves it",
        "Too lazy to write descriptive message",
        "Too tired to write descriptive message",
        "Treid to fix some stuff ddint go well",
        "Trying empty commit",
        "WIP",
        "WTF is this?",
        "What the hell...",
        "Whatever",
        "Who the hell used 8-space tabs?",
        "Why the hell not?",
        "Why you are looking at MY commits?",
        "Works for me",
        "YOLO",
        "Yeah, it doesn't work. And?",
        "Yer fokin cunt its werkin now",
        "Yes",
        "You are not expected to understand this",
        "You're gonna love this one",
        "asdasdasda",
        "fix bug",
        "kek",
        "yikes",
        "yuck"
    ];
}