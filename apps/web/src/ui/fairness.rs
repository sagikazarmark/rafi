//! "How fairness works": the draw explained in plain words, with diagrams.
//!
//! What it says follows the draw as built (docs/explanation/verifiability.md
//! has the same, in full): the list frozen and its hash published, each draw
//! committed to a future drand round, the winner computed by a public formula,
//! the rules around it, and where the guarantees end.

use dioxus::prelude::*;
use dioxus_icons::lucide::{
    ArrowDown, ArrowRight, BadgeCheck, Ban, Building2, Calculator, CircleCheck, CircleX, Dice5,
    EyeOff, FingerprintPattern, Hand, Hash, History, Hourglass, Info, ListOrdered, Lock,
    LockKeyhole, Mail, Megaphone, QrCode, Radio, RefreshCw, Search, ShieldCheck, Ticket, Trophy,
    UserCheck, Users,
};

use super::{Brand, Colophon, Route, ThemeToggle};

#[component]
pub fn Fairness() -> Element {
    let build = use_hook(crate::config::build);
    let verifier = build.verifier_url.clone();
    rsx! {
        document::Title { "How fairness works — Rafi" }
        div { class: "min-h-screen bg-base-100 bg-dots",
            nav { class: "mx-auto flex w-full max-w-6xl items-center gap-3 px-5 py-4",
                Link { to: Route::Home {}, Brand {} }
                span { class: "flex-1" }
                ThemeToggle {}
                Link { class: "btn btn-sm btn-ghost", to: Route::AdminHome {}, "Organizer login" }
            }
            main { class: "mx-auto max-w-5xl space-y-20 px-5 pb-20 pt-8 sm:space-y-28 sm:pt-12",
                Intro {}
                ShortVersion {}
                WhyNobodyCanCheat {}
                WhereTheNumberComesFrom {}
                FromNumberToWinner {}
                TheRules {}
                TheOrganizer {}
                CheckItYourself { verifier }
                PrivacyAndLimits {}
                Closing {}
            }
            Colophon { info: build }
        }
    }
}

/// A section's heading: a small label above, the title, and a line or two
/// under it.
#[component]
fn Heading(label: String, title: String, #[props(default)] lead: Option<String>) -> Element {
    rsx! {
        div { class: "mx-auto max-w-2xl space-y-3 text-center",
            p { class: "text-sm font-semibold uppercase tracking-widest text-primary", "{label}" }
            h2 { class: "text-3xl font-extrabold leading-tight sm:text-4xl", "{title}" }
            if let Some(lead) = lead {
                p { class: "text-lg text-base-content/70", "{lead}" }
            }
        }
    }
}

/// An arrow from one part of a diagram to the next: down when the diagram
/// stacks, right from `lg` (or `md`) up.
#[component]
fn Flow(#[props(default = "lg".to_string())] from: String) -> Element {
    let (down, right) = if from == "md" {
        ("md:hidden", "hidden md:block")
    } else {
        ("lg:hidden", "hidden lg:block")
    };
    rsx! {
        div { class: "grid place-items-center text-base-content/40", aria_hidden: "true",
            ArrowDown { class: "size-6 {down}" }
            ArrowRight { class: "size-6 {right}" }
        }
    }
}

const CARD: &str = "card bg-base-100 shadow-sm ring-1 ring-base-300";

#[component]
fn Intro() -> Element {
    rsx! {
        header { class: "mx-auto max-w-3xl space-y-6 text-center",
            span { class: "badge badge-soft badge-secondary gap-1.5 font-semibold",
                ShieldCheck { class: "size-3.5" }
                "How fairness works"
            }
            h1 { class: "text-4xl font-extrabold leading-[1.05] sm:text-6xl",
                "Nobody picks the winner. "
                span { class: "gradient-text", "Not even the organizer." }
            }
            p { class: "mx-auto max-w-2xl text-lg text-base-content/70",
                "Every prize is drawn from a random number that doesn't exist yet when the draw starts. Anyone can check the result afterwards. Here's how, in plain words."
            }
        }
    }
}

#[component]
fn ShortVersion() -> Element {
    rsx! {
        section { class: "space-y-10",
            Heading { label: "The short version", title: "Three steps, all in public" }
            div { class: "grid items-stretch gap-3 md:grid-cols-[1fr_auto_1fr_auto_1fr]",
                Step { n: 1, icon: rsx! { Lock { class: "size-7" } }, title: "The list is locked",
                    body: "Before the first draw, the list of entrants is frozen and its fingerprint is published. Nobody can join, leave or be added after that."
                }
                Flow { from: "md" }
                Step { n: 2, icon: rsx! { Hourglass { class: "size-7" } }, title: "A future number is chosen",
                    body: "Each draw names a random number that will be published a few seconds later. Nobody knows it yet, not even Rafi."
                }
                Flow { from: "md" }
                Step { n: 3, icon: rsx! { Trophy { class: "size-7" } }, title: "The number picks the winner",
                    body: "When the number comes out, a public formula turns it into a winner. Anyone can redo the math and get the same name."
                }
            }
            Fingerprint {}
        }
    }
}

#[component]
fn Step(n: u8, icon: Element, title: String, body: String) -> Element {
    rsx! {
        div { class: "{CARD}",
            div { class: "card-body items-center gap-3 text-center",
                div { class: "relative",
                    span { class: "grid size-14 place-items-center rounded-2xl bg-primary/10 text-primary", {icon} }
                    span { class: "badge badge-primary badge-sm absolute -right-3 -top-2 font-bold", "{n}" }
                }
                h3 { class: "card-title text-xl", "{title}" }
                p { class: "text-base-content/70", "{body}" }
            }
        }
    }
}

/// What the list's fingerprint (the manifest hash) is, with a list and one
/// changed entry side by side.
#[component]
fn Fingerprint() -> Element {
    rsx! {
        div { class: "{CARD}",
            div { class: "card-body grid items-center gap-8 md:grid-cols-2",
                div { class: "space-y-3",
                    h3 { class: "flex items-center gap-2 text-xl font-bold",
                        FingerprintPattern { class: "size-6 text-primary" }
                        "What's a fingerprint?"
                    }
                    p { class: "text-base-content/70",
                        "A short code computed from the locked list and the prizes. Change a single entry and the code changes completely."
                    }
                    p { class: "text-base-content/70",
                        "It's published before the first draw, so the list is pinned down: if anyone was added or removed later, the code wouldn't match, and every check would fail."
                    }
                }
                div { class: "space-y-4", aria_hidden: "true",
                    FingerprintRow { entries: ["a41f…", "7c02…", "e9b3…"], changed: false, code: "3f9a 71c2" }
                    FingerprintRow { entries: ["a41f…", "7c03…", "e9b3…"], changed: true, code: "b05e 9d14" }
                }
            }
        }
    }
}

#[component]
fn FingerprintRow(entries: [&'static str; 3], changed: bool, code: &'static str) -> Element {
    let code_class = if changed {
        "bg-error/10 text-error ring-error/30"
    } else {
        "bg-primary/10 text-primary ring-primary/30"
    };
    rsx! {
        div { class: "flex items-center gap-3",
            div { class: "flex-1 space-y-1 rounded-box bg-base-200 p-3 font-mono text-sm",
                for (i, e) in entries.iter().enumerate() {
                    div { class: if changed && i == 1 { "rounded bg-error/15 px-1.5 text-error" } else { "px-1.5" }, "{e}" }
                }
            }
            ArrowRight { class: "size-5 shrink-0 text-base-content/40" }
            div { class: "flex-1 rounded-box px-3 py-4 text-center font-mono text-lg font-bold ring-1 {code_class}",
                "{code}"
            }
        }
    }
}

#[component]
fn WhyNobodyCanCheat() -> Element {
    rsx! {
        section { class: "space-y-10",
            Heading {
                label: "Why nobody can cheat",
                title: "The winner comes from a number nobody has seen yet",
                lead: "Rafi commits to the number before it exists. By the time it exists, everything else is already fixed and public.",
            }
            div { class: "{CARD}",
                div { class: "card-body",
                    // Titles as tall as they are (daisyUI shares the height out evenly).
                    ul { class: "timeline timeline-vertical max-lg:timeline-compact lg:timeline-horizontal lg:*:flex-1 lg:*:[--timeline-row-start:auto]",
                        Moment { first: true, icon: rsx! { Lock { class: "size-5" } }, title: "List locked",
                            body: "Its fingerprint is published."
                        }
                        Moment { icon: rsx! { Megaphone { class: "size-5" } }, title: "Draw starts", waiting_after: true,
                            body: "The screen shows which drand round will decide it."
                        }
                        Moment { icon: rsx! { Hourglass { class: "size-5" } }, title: "Countdown", waiting: true, waiting_before: true, waiting_after: true,
                            body: "A few seconds. Nobody knows the number, not even Rafi."
                        }
                        Moment { icon: rsx! { Radio { class: "size-5" } }, title: "Number published", waiting_before: true,
                            body: "drand publishes it, signed."
                        }
                        Moment { last: true, icon: rsx! { Trophy { class: "size-5" } }, title: "Winner",
                            body: "The formula picks them. Anyone can redo it."
                        }
                    }
                }
            }
            div { class: "grid gap-4 md:grid-cols-2",
                Callout { icon: rsx! { EyeOff { class: "size-6" } }, tone: "accent", title: "Before the number exists",
                    body: "Nobody can know who will win, so there's nothing to aim for. Pressing Draw at a \"good\" moment changes nothing."
                }
                Callout { icon: rsx! { Lock { class: "size-6" } }, tone: "primary", title: "Once it exists",
                    body: "The list, the round and the formula are already public, so nobody can change whom it picks."
                }
            }
        }
    }
}

/// One moment of the draw's timeline. The line between "Draw starts" and
/// "Number published" is the waiting, in the accent colour.
#[component]
fn Moment(
    icon: Element,
    title: String,
    body: String,
    #[props(default)] first: bool,
    #[props(default)] last: bool,
    #[props(default)] waiting: bool,
    #[props(default)] waiting_before: bool,
    #[props(default)] waiting_after: bool,
) -> Element {
    let line = |w: bool| if w { "bg-accent" } else { "bg-primary/40" };
    let dot = if waiting {
        "bg-accent text-accent-content"
    } else {
        "bg-primary text-primary-content"
    };
    rsx! {
        li {
            if !first { hr { class: line(waiting_before) } }
            div { class: "timeline-start font-display font-bold lg:mb-2", "{title}" }
            div { class: "timeline-middle",
                span { class: "grid size-10 place-items-center rounded-full {dot}", {icon} }
            }
            div { class: "timeline-end timeline-box text-sm text-base-content/70 lg:mt-2 lg:w-40 lg:text-center", "{body}" }
            if !last { hr { class: line(waiting_after) } }
        }
    }
}

#[component]
fn Callout(icon: Element, tone: &'static str, title: String, body: String) -> Element {
    let badge = match tone {
        // Solid: accent-content is dark in both themes, unreadable on a faint accent in the dark one.
        "accent" => "bg-accent text-accent-content",
        _ => "bg-primary/10 text-primary",
    };
    rsx! {
        div { class: "{CARD}",
            div { class: "card-body flex-row items-start gap-4",
                span { class: "grid size-11 shrink-0 place-items-center rounded-2xl {badge}", {icon} }
                div { class: "space-y-1",
                    h3 { class: "text-lg font-bold", "{title}" }
                    p { class: "text-base-content/70", "{body}" }
                }
            }
        }
    }
}

#[component]
fn WhereTheNumberComesFrom() -> Element {
    rsx! {
        section { class: "space-y-10",
            Heading {
                label: "Where the number comes from",
                title: "drand: randomness nobody owns",
                lead: "drand is run by the League of Entropy: independent universities, companies and foundations around the world. Together, they publish a new random number every 3 seconds.",
            }
            div { class: "grid items-center gap-6 lg:grid-cols-2",
                Beacon {}
                div { class: "grid gap-4 sm:grid-cols-2",
                    Property { icon: rsx! { EyeOff { class: "size-5" } }, title: "Unpredictable",
                        body: "Nobody can work out a number before its time. It takes many members together to make one."
                    }
                    Property { icon: rsx! { Ban { class: "size-5" } }, title: "Can't be steered",
                        body: "Each round has exactly one valid number, so nobody can try again until they like the result."
                    }
                    Property { icon: rsx! { BadgeCheck { class: "size-5" } }, title: "Checkable",
                        body: "Every number is signed by the network. Anyone can check the signature, so Rafi can't fake one."
                    }
                    Property { icon: rsx! { Users { class: "size-5" } }, title: "Independent",
                        body: "The randomness doesn't come from the organizer or from Rafi, and no single member of the network controls it."
                    }
                }
            }
            p { class: "text-center",
                a { class: "link link-primary", href: "https://drand.love", target: "_blank", rel: "noopener", "More about drand" }
            }
        }
    }
}

/// The network making a round: members each adding their part, and the
/// round they make together.
#[component]
fn Beacon() -> Element {
    rsx! {
        div { class: "{CARD}", aria_hidden: "true",
            div { class: "card-body items-center gap-4",
                div { class: "flex flex-wrap justify-center gap-2",
                    for who in ["University", "Company", "Foundation", "University", "Company"] {
                        span { class: "badge badge-outline gap-1.5 py-3",
                            Building2 { class: "size-3.5" }
                            "{who}"
                        }
                    }
                }
                div { class: "flex flex-col items-center gap-1 text-center text-sm text-base-content/60",
                    ArrowDown { class: "size-5" }
                    "Each adds its part. A number only exists once enough of them have."
                    ArrowDown { class: "size-5" }
                }
                div { class: "ticket w-full max-w-xs bg-primary p-6 text-primary-content shadow-lg",
                    p { class: "text-xs font-semibold uppercase tracking-widest opacity-80", "drand round 33712045" }
                    p { class: "mt-1 font-mono text-xl font-bold tracking-wider", "9f3c 41e7 … 08b2" }
                    div { class: "my-4 border-t-2 border-dashed border-primary-content/40" }
                    p { class: "flex items-center gap-1.5 text-sm font-semibold",
                        BadgeCheck { class: "size-4" }
                        "Signed by the network"
                    }
                }
            }
        }
    }
}

#[component]
fn Property(icon: Element, title: String, body: String) -> Element {
    rsx! {
        div { class: "{CARD}",
            div { class: "card-body gap-2 p-5",
                h3 { class: "flex items-center gap-2 font-bold",
                    span { class: "text-primary", {icon} }
                    "{title}"
                }
                p { class: "text-sm text-base-content/70", "{body}" }
            }
        }
    }
}

#[component]
fn FromNumberToWinner() -> Element {
    rsx! {
        section { class: "space-y-10",
            Heading {
                label: "From number to winner",
                title: "A public formula, the same for everyone",
                lead: "Rafi's server doesn't choose the winner. The winner is computed from three public ingredients, and the math gives the same answer on any computer.",
            }
            div { class: "{CARD}",
                div { class: "card-body",
                    div { class: "grid items-center gap-4 lg:grid-cols-[1fr_auto_auto_auto_1fr]", aria_hidden: "true",
                        div { class: "space-y-2",
                            Ingredient { icon: rsx! { Radio { class: "size-5" } }, label: "drand number", value: "9f3c…08b2" }
                            p { class: "text-center font-bold text-base-content/40", "+" }
                            Ingredient { icon: rsx! { FingerprintPattern { class: "size-5" } }, label: "List fingerprint", value: "3f9a…71c2" }
                            p { class: "text-center font-bold text-base-content/40", "+" }
                            Ingredient { icon: rsx! { Hash { class: "size-5" } }, label: "Draw number", value: "3" }
                        }
                        Flow {}
                        div { class: "mx-auto grid size-36 place-items-center rounded-full bg-primary/10 text-center text-primary ring-2 ring-primary/30",
                            div { class: "space-y-1",
                                Calculator { class: "mx-auto size-8" }
                                p { class: "font-display font-bold", "Public formula" }
                            }
                        }
                        Flow {}
                        div { class: "flex flex-col items-center gap-2",
                            div { class: "flex items-center gap-2 rounded-box bg-base-200 px-4 py-3",
                                ListOrdered { class: "size-5 text-primary" }
                                span { class: "font-semibold", "No. 42 of 120 in the draw" }
                            }
                            ArrowDown { class: "size-5 text-base-content/40" }
                            div { class: "ticket w-full max-w-56 bg-primary px-6 py-4 text-center text-primary-content shadow-lg",
                                p { class: "text-xs font-semibold uppercase tracking-widest opacity-80", "Winner" }
                                p { class: "font-display text-2xl font-extrabold", "Alex" }
                            }
                        }
                    }
                }
            }
            div { class: "grid gap-4 md:grid-cols-3",
                Property { icon: rsx! { Radio { class: "size-5" } }, title: "The drand number",
                    body: "The randomness. Nobody could know it when the draw started."
                }
                Property { icon: rsx! { FingerprintPattern { class: "size-5" } }, title: "The list's fingerprint",
                    body: "Ties the result to this raffle and this exact list."
                }
                Property { icon: rsx! { Hash { class: "size-5" } }, title: "The draw number",
                    body: "Each draw gets its own, so no two draws are alike."
                }
            }
            Callout { icon: rsx! { Dice5 { class: "size-6" } }, tone: "primary", title: "The same chance for everyone",
                body: "It works like a die with one side for each person still in the draw. When a roll would favour some of them over others, it's thrown away and rolled again, so everyone has exactly the same chance."
            }
        }
    }
}

#[component]
fn Ingredient(icon: Element, label: String, value: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-3 rounded-box bg-base-200 px-4 py-3",
            span { class: "text-primary", {icon} }
            span { class: "flex-1 font-semibold", "{label}" }
            span { class: "font-mono text-sm text-base-content/70", "{value}" }
        }
    }
}

#[component]
fn TheRules() -> Element {
    rsx! {
        section { class: "space-y-10",
            Heading { label: "The rules", title: "Rules that leave no room for tricks" }
            div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                Rule { icon: rsx! { UserCheck { class: "size-6" } }, title: "One prize per person",
                    body: "Once you're drawn, you're out of the pool. Only if there are more prizes than people can someone win twice."
                }
                Rule { icon: rsx! { RefreshCw { class: "size-6" } }, title: "No rerolls",
                    body: "If the number is slow to arrive, the draw waits for that same number. Trying again can't change the result."
                }
                Rule { icon: rsx! { Hand { class: "size-6" } }, title: "Winners claim in person",
                    body: "Winners must be at the event to take their prize. Nobody walks up for a name that was made up."
                }
                Rule { icon: rsx! { Megaphone { class: "size-6" } }, title: "Redraws happen in public",
                    body: "If a winner isn't there or says no, they're out, and the prize is drawn again with a new number. The record shows both."
                }
                Rule { icon: rsx! { History { class: "size-6" } }, title: "Nothing disappears",
                    body: "Every draw, redraw and withdrawn prize stays on the raffle's public record. Once the first draw starts, the raffle can't be deleted."
                }
                Rule { icon: rsx! { Mail { class: "size-6" } }, title: "One entry per person",
                    body: "One entry per email address, with a quick check that you're not a bot when you join."
                }
            }
        }
    }
}

#[component]
fn Rule(icon: Element, title: String, body: String) -> Element {
    rsx! {
        div { class: "{CARD}",
            div { class: "card-body gap-3",
                span { class: "grid size-11 place-items-center rounded-2xl bg-primary/10 text-primary", {icon} }
                h3 { class: "card-title text-lg", "{title}" }
                p { class: "text-base-content/70", "{body}" }
            }
        }
    }
}

#[component]
fn TheOrganizer() -> Element {
    rsx! {
        section { class: "space-y-10",
            Heading { label: "The organizer", title: "What the organizer can and can't do" }
            div { class: "grid gap-4 md:grid-cols-2",
                div { class: "{CARD}",
                    div { class: "card-body gap-4",
                        h3 { class: "flex items-center gap-2 text-xl font-bold",
                            CircleCheck { class: "size-6 text-success" }
                            "Can"
                        }
                        ul { class: "space-y-3",
                            Can { "Set up the prizes and the list, until the draw starts" }
                            Can { "Add people who don't have a phone. The record counts them separately." }
                            Can { "Choose which prize to draw next" }
                            Can { "Mark a winner as not here, or as declining. Both show on the record." }
                            Can { "Withdraw a prize that hasn't been drawn. That shows on the record too." }
                            Can { "Hide or rename names on the public list" }
                        }
                    }
                }
                div { class: "{CARD}",
                    div { class: "card-body gap-4",
                        h3 { class: "flex items-center gap-2 text-xl font-bold",
                            CircleX { class: "size-6 text-error" }
                            "Can't"
                        }
                        ul { class: "space-y-3",
                            Cannot { "Pick the winner, or the number" }
                            Cannot { "Add or remove anyone once the list is locked" }
                            Cannot { "Draw a prize again without it showing on the record" }
                            Cannot { "Get a different number by pressing Draw again" }
                            Cannot { "Delete the raffle once the first draw has started" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Can(children: Element) -> Element {
    rsx! {
        li { class: "flex items-start gap-3",
            CircleCheck { class: "mt-0.5 size-5 shrink-0 text-success" }
            span { {children} }
        }
    }
}

#[component]
fn Cannot(children: Element) -> Element {
    rsx! {
        li { class: "flex items-start gap-3",
            CircleX { class: "mt-0.5 size-5 shrink-0 text-error" }
            span { {children} }
        }
    }
}

#[component]
fn CheckItYourself(verifier: Option<String>) -> Element {
    rsx! {
        section { id: "check", class: "space-y-10",
            Heading {
                label: "Don't take our word for it",
                title: "Check every draw yourself",
                lead: "Every raffle has a public verification page. Open it, and your browser re-runs every draw from the published list and the drand numbers.",
            }
            div { class: "grid items-stretch gap-3 md:grid-cols-[1fr_auto_1fr_auto_1fr]",
                Step { n: 1, icon: rsx! { QrCode { class: "size-7" } }, title: "Open the page",
                    body: "Scan the QR code on the screen at the end, or follow the link from your entry on your phone."
                }
                Flow { from: "md" }
                Step { n: 2, icon: rsx! { Calculator { class: "size-7" } }, title: "Your browser redoes the math",
                    body: "It checks the list's fingerprint, every drand signature, and every winner."
                }
                Flow { from: "md" }
                Step { n: 3, icon: rsx! { Search { class: "size-7" } }, title: "Find your name",
                    body: "See that you were in the locked list, and how each prize was drawn."
                }
            }
            p { class: "mx-auto max-w-2xl text-center text-base-content/70",
                "Prefer your own tools? Download the record from the verification page and run the "
                if let Some(url) = verifier {
                    a { class: "link link-primary", href: "{url}", target: "_blank", rel: "noopener", "independent verifier" }
                } else {
                    "independent verifier"
                }
                ", a short Python script."
            }
        }
    }
}

#[component]
fn PrivacyAndLimits() -> Element {
    rsx! {
        section { class: "grid gap-4 md:grid-cols-2",
            div { class: "{CARD}",
                div { class: "card-body gap-4",
                    h3 { class: "flex items-center gap-2 text-xl font-bold",
                        LockKeyhole { class: "size-6 text-primary" }
                        "Your data stays private"
                    }
                    ul { class: "list-disc space-y-2 pl-5 text-base-content/70",
                        li { "The draw only uses random, anonymous IDs." }
                        li { "Your email address is never published." }
                        li { "Your name and email are erased on a date you see before you join (a winner's, once their prize is delivered). The IDs stay, so the draw can still be checked." }
                    }
                }
            }
            div { class: "{CARD}",
                div { class: "card-body gap-4",
                    h3 { class: "flex items-center gap-2 text-xl font-bold",
                        Info { class: "size-6 text-primary" }
                        "Where the guarantees end"
                    }
                    ul { class: "list-disc space-y-2 pl-5 text-base-content/70",
                        li { "The draw is fair over the list it's given. Until the list is locked, the organizer can add people. The verification page counts who joined on their phones and who the organizer added, and lists everyone (by name, unless a name is hidden) until their data is erased." }
                        li { "If drand can't be reached, the organizer can abandon a draw and draw that prize again with a new number. It's the one lever left, and it's public: the verification page lists every abandoned draw, and the independent verifier shows whom its number would have picked." }
                    }
                }
            }
        }
    }
}

#[component]
fn Closing() -> Element {
    rsx! {
        section { class: "{CARD}",
            div { class: "card-body items-center gap-4 py-10 text-center",
                h2 { class: "text-3xl font-extrabold", "Fair, and fun to watch" }
                p { class: "max-w-xl text-base-content/70",
                    "The countdown before each number is the suspense. Everyone in the room sees the same draw, and anyone can check it afterwards."
                }
                div { class: "flex flex-wrap justify-center gap-3",
                    Link { class: "btn btn-primary btn-lg hover-wiggle", to: Route::AdminHome {},
                        Ticket { class: "size-5" }
                        "Start a raffle"
                    }
                    Link { class: "btn btn-ghost btn-lg", to: Route::Home {}, "Back home" }
                }
            }
        }
    }
}
