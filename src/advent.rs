/* LCG PRNG parameters tested against
 * Knuth vol. 2. by the original authors */
const LCG_A: i32 = 1093;
const LCG_C: i32 = 221587;
const LCG_M: i32 = 1048576;

const LINESIZE: i32 = 1024;
pub static TOKLEN: i32 = 5;            // # sigificant characters in a token */
const NDWARVES: i32 = 6;          // number of dwarves
const PIRATE: i32 = NDWARVES;     // must be NDWARVES-1 when zero-origin
// THE FOLLOWING DATATYPE MAY BE WRONG
const DALTLC: &str = LOC_NUGGET;  // alternate dwarf location

const INVLIMIT: i32 = 7;          // inventory limit (# of objects)
const INTRANSITIVE: i32 = -1;     // illegal object number
const GAMELIMIT: i32 = 330;       // base limit of turns
const NOVICELIMIT: i32 = 1000;    // limit of turns for novice
const WARNTIME: i32 = 30;         // late game starts at game.limit-this
const FLASHTIME: i32 = 50;        // turns from first warning till blinding flash
const PANICTIME: i32 = 15;        // time left after closing
const BATTERYLIFE: i32 = 2500;    // turn limit increment from batteries
const WORD_NOT_FOUND: i32 = -1;   // "Word not found" flag value for the vocab hash functions.
const WORD_EMPTY: i32 = 0;        // "Word empty" flag value for the vocab hash functions
const CARRIED: i32 = -1;          // Player is toting it
const READ_MODE: &str = "rb";     // b is not needed for POSIX but harmless
const WRITE_MODE: &str = "wb";    // b is not needed for POSIX but harmless

/* Special object-state values - integers > 0 are object-specific */
const STATE_NOTFOUND: i32 = -1;  // 'Not found" state of treasures */
const STATE_FOUND: i32 = 0;	     // After discovered, before messed with
const STATE_IN_CAVITY: i32 = 1;	 // State value common to all gemstones

/* Special fixed object-state values - integers > 0 are location */
const IS_FIXED: i32 = -1;
const IS_FREE: i32 = 0;

/* Map a state property value to a negative range, where the object cannot be
 * picked up but the value can be recovered later.  Avoid colliding with -1,
 * which has its own meaning. */

// STASHED(obj)	(-1 - game.prop[obj])
macro_rules! STASHED {
    ($obj:expr) => {
        (-1 - game.prop[$obj])
    };
}

/*
 *  DESTROY(N)  = Get rid of an item by putting it in LOC_NOWHERE
 *  MOD(N,M)    = Arithmetic modulus
 *  TOTING(OBJ) = true if the OBJ is being carried
 *  AT(OBJ)     = true if on either side of two-placed object
 *  HERE(OBJ)   = true if the OBJ is at "LOC" (or is being carried)
 *  CNDBIT(L,N) = true if COND(L) has bit n set (bit 0 is units bit)
 *  LIQUID()    = object number of liquid in bottle
 *  LIQLOC(LOC) = object number of liquid (if any) at LOC
 *  FORCED(LOC) = true if LOC moves without asking for input (COND=2)
 *  DARK(LOC)   = true if location "LOC" is dark
 *  PCT(N)      = true N% of the time (N integer from 0 to 100)
 *  GSTONE(OBJ) = true if OBJ is a gemstone
 *  FOREST(LOC) = true if LOC is part of the forest
 *  OUTSID(LOC) = true if location not in the cave
 *  INSIDE(LOC) = true if location is in the cave or the building at the beginning of the game
 *  INDEEP(LOC) = true if location is in the Hall of Mists or deeper
 *  BUG(X)      = report bug and exit
 */

//  #define DESTROY(N)   move(N, LOC_NOWHERE)
macro_rules! DESTROY {
    ($n:expr) => {
        move($n, LOC_NOWHERE) 
    };
}

// #define MOD(N,M)     ((N) % (M))
macro_rules! MOD {
    ($n:expr, $m:expr) => {
        ($n % $m)
    };
}

// #define TOTING(OBJ)  (game.place[OBJ] == CARRIED)
macro_rules! TOTING {
    ($obj:expr) => {
        (game.place[$obj] == CARRIED)
    };
}

// #define AT(OBJ)      (game.place[OBJ] == game.loc || game.fixed[OBJ] == game.loc)
macro_rules! AT {
    ($obj:expr) => {
        (game.place[$obj] == game.loc || game.fixed[$obj] == game.loc)
    };
}

// #define HERE(OBJ)    (AT(OBJ) || TOTING(OBJ))
macro_rules! HERE {
    ($obj:expr) => {
        (AT($obj) || TOTING($obj))
    };
}

// #define CNDBIT(L,N)  (tstbit(conditions[L],N))
macro_rules! CNDBIT {
    ($l:expr, $n:expr) => {
        tstbit(conditions[$l], $n)
    };
}

// #define LIQUID()     (game.prop[BOTTLE] == WATER_BOTTLE? WATER : game.prop[BOTTLE] == OIL_BOTTLE ? OIL : NO_OBJECT )
macro_rules! LIQUID {
    () => {
        (game.prop[BOTTLE] == WATER_BOTTLE? WATER : game.prop[BOTTLE] == OIL_BOTTLE ? OIL : NO_OBJECT )
    };
}

// #define LIQLOC(LOC)  (CNDBIT((LOC),COND_FLUID)? CNDBIT((LOC),COND_OILY) ? OIL : WATER : NO_OBJECT)
macro_rules! LIQLOC {
    ($loc:expr) => {
        (CNDBIT($loc, COND_FLUID)? CNDBIT($loc, COND_OILY) ? OIL : WATER : NO_OBJECT)
    };
}

// #define FORCED(LOC)  CNDBIT(LOC, COND_FORCED)
macro_rules! FORCED {
    ($loc:expr) => {
        CNDBIT($loc, COND_FORCED)
    };
}

// #define DARK(DUMMY)  (!CNDBIT(game.loc,COND_LIT) && (game.prop[LAMP] == LAMP_DARK || !HERE(LAMP)))
macro_rules! DARK {
    ($dummy:expr) => {
        (!CNDBIT(game.loc,COND_LIT) && (game.prop[LAMP] == LAMP_DARK || !HERE(LAMP)))
    };
}

// #define PCT(N)       (randrange(100) < (N))
macro_rules! PCT {
    ($n:expr) => {
        (randrange(100) < $n)
    };
}

// #define GSTONE(OBJ)  ((OBJ) == EMERALD || (OBJ) == RUBY || (OBJ) == AMBER || (OBJ) == SAPPH)
macro_rules! GSTONE {
    ($obj:expr) => {
        ($obj == EMERALD || $obj == RUBY || $obj == AMBER || $obj == SAPPH)
    };
}

// #define FOREST(LOC)  CNDBIT(LOC, COND_FOREST)
macro_rules! FOREST {
    ($loc:expr) => {
        CNDBIT($loc, COND_FOREST)
    };
}

// #define OUTSID(LOC)  (CNDBIT(LOC, COND_ABOVE) || FOREST(LOC))
macro_rules! OUTSID {
    ($loc:expr) => {
        (CNDBIT($loc, COND_ABOVE) || FOREST($loc))
    };
}

// #define INSIDE(LOC)  (!OUTSID(LOC) || LOC == LOC_BUILDING)
macro_rules! INSIDE {
    ($loc:expr) => {
        (!OUTSID($loc) || $loc == LOC_BUILDING)
    };
}

// #define INDEEP(LOC)  ((LOC) >= LOC_MISTHALL && !OUTSID(LOC))
macro_rules! INDEEP {
    ($loc:expr) => {
        ($loc >= LOC_MISTHALL && !OUTSID($loc))
    };
}

// #define BUG(x)       bug(x, #x)
macro_rules! BUG {
    ($x:expr) => {
        bug($x, stringify!($x))
    };
}


enum bugtype {
    ACTION_RETURNED_PHASE_CODE_BEYOND_END_OF_SWITCH,
    CONDITIONAL_TRAVEL_ENTRY_WITH_NO_ALTERATION,
    HINT_NUMBER_EXCEEDS_GOTO_LIST,
    INTRANSITIVE_ACTION_VERB_EXCEEDS_GOTO_LIST,
    LOCATION_HAS_NO_TRAVEL_ENTRIES,
    SPECIAL_TRAVEL_500_GT_L_GT_300_EXCEEDS_GOTO_LIST,
    SPEECHPART_NOT_TRANSITIVE_OR_INTRANSITIVE_OR_UNKNOWN,
    TRANSITIVE_ACTION_VERB_EXCEEDS_GOTO_LIST,
    VOCABULARY_TYPE_N_OVER_1000_NOT_BETWEEN_0_AND_3,
}

enum speaktype {change, hear, look, study, touch}

enum termination {endgame, quitgame, scoregame}

enum speechpart {unknown, intransitive, transitive}

// typedef enum {NO_WORD_TYPE, MOTION, OBJECT, ACTION, NUMERIC} word_type_t;
pub enum word_type_t {NO_WORD_TYPE, MOTION, OBJECT, ACTION, NUMERIC}

// typedef enum scorebonus {none, splatter, defeat, victory} score_t;
pub enum scorebonus {none, splatter, defeat, victory}

/* Phase codes for action returns.
 * These were at one time FORTRAN line numbers.
 * The values don't matter, but perturb their order at your peril.
 */
pub enum phase_codes_t {
    GO_TERMINATE,
    GO_MOVE,
    GO_TOP,
    GO_CLEAROBJ,
    GO_CHECKHINT,
    GO_WORD2,
    GO_UNKNOWN,
    GO_DWARFWAKE,
}

pub static mut vocab_t: i32;  // index into a vocabulary array */
pub static mut verb_t: i32;   // index into an actions array */
pub static mut obj_t: i32;    // index into the object array */
pub static mut loc_t: i32;    // index into the locations array */
pub static mut turn_t: i31;   // turn counter or threshold */

struct game_t {
    lcg_x: i32,
    abbnum: i32,                  // How often to print int descriptions
    bonus: score_t,               // What kind of finishing bonus we are getting
    chloc: loc_t,                 // pirate chest location
    chloc2: loc_t,                // pirate chest alternate location
    clock1: turn_t,               // # turns from finding last treasure to close
    clock2: turn_t,               // # turns from warning till blinding flash
    clshnt: bool,                 // has player read the clue in the endgame?
    closed: bool,                 // whether we're all the way closed
    closng: bool,                 // whether it's closing time yet
    lmwarn: bool,                 // has player been warned about lamp going dim?
    novice: bool,                 // asked for instructions at start-up?
    panic: bool,                  // has player found out he's trapped?
    wzdark: bool,                 // whether the loc he's leaving was dark
    blooded: bool,                // has player drunk of dragon's blood?
    conds: i32,                  // min value for cond[loc] if loc has any hints
    detail: i32,                  // level of detail in descriptions

    /*  dflag controls the level of activation of dwarves:
     *	0	No dwarf stuff yet (wait until reaches Hall Of Mists)
     *	1	Reached Hall Of Mists, but hasn't met first dwarf
     *	2	Met first dwarf, others start moving, no knives thrown yet
     *	3	A knife has been thrown (first set always misses)
     *	3+	Dwarves are mad (increases their accuracy) */
    dflag: i32,

    dkill: i32,                   // dwarves killed
    dtotal: i32,                  // total dwarves (including pirate) in loc
    foobar: i32,                  // progress in saying "FEE FIE FOE FOO".
    holdng: i32,                  // number of objects being carried
    igo: i32,                     // # uses of "go" instead of a direction
    iwest: i32,                   // # times he's said "west" instead of "w"
    knfloc: i32,                  // knife location; 0 if none, -1 after caveat
    limit: turn_t,                // lifetime of lamp
    loc: loc_t,                   // where player is now
    newloc: loc_t,                // where player is going
    numdie: turn_t,               // number of times killed so far
    oldloc: loc_t,                // where player was
    oldlc2: loc_t,                // where player was two moves ago
    oldobj: obj_t,                // last object player handled
    saved: i32,                   // point penalty for saves
    tally: i32,                   // count of treasures gained
    thresh: i32,                  // current threshold for endgame scoring tier
    trndex: turn_t,               // FIXME: not used, remove on next format bump
    trnluz: turn_t,               // # points lost so far due to turns used
    turns: turn_t,                // counts commands given (ignores yes/no)
    zzword[TOKLEN + 1]: &char,    // randomly generated magic word from bird
    abbrev[NLOCATIONS + 1]: i32,  // has location been seen?
    int atloc[NLOCATIONS + 1];   // head of object linked list per location
    int dseen[NDWARVES + 1];     // true if dwarf has seen him
    loc_t dloc[NDWARVES + 1];    // location of dwarves, initially hard-wired in
    loc_t odloc[NDWARVES + 1];   // prior loc of each dwarf, initially garbage
    loc_t fixed[NOBJECTS + 1];   // fixed location of object (if  not IS_FREE)
    obj_t link[NOBJECTS * 2 + 1];// object-list links
    loc_t place[NOBJECTS + 1];   // location of object
    int hinted[NHINTS];          // hinted[i] = true iff hint i has been used.
    int hintlc[NHINTS];          // hintlc[i] = how int at LOC with cond bit i
    int prop[NOBJECTS + 1];      // object state array */
};

/*
 * Game application settings - settings, but not state of the game, per se.
 * This data is not saved in a saved game.
 */
struct settings_t {
    *logfp: FILE;
    oldstyle: bool;
    prompt: bool;
}

pub struct command_word_t {
    raw[LINESIZE]: char;
    id: vocab_t;
    type: word_type_t;
} 

pub enum command_state_t {EMPTY, RAW, TOKENIZED, GIVEN, PREPROCESSED, PROCESSING, EXECUTED}

typedef struct {
    enum speechpart part;
    command_word_t word[2];
    verb_t verb;
    obj_t obj;
    command_state_t state;
} command_t;

extern struct game_t game;
extern struct settings_t settings;

extern bool get_command_input(command_t *);
extern void clear_command(command_t *);
extern void speak(const char*, ...);
extern void sspeak(int msg, ...);
extern void pspeak(vocab_t, enum speaktype, bool, int, ...);
extern void rspeak(vocab_t, ...);
extern void echo_input(FILE*, const char*, const char*);
extern bool silent_yes(void);
extern bool yes(const char*, const char*, const char*);
extern void juggle(obj_t);
extern void move(obj_t, loc_t);
extern loc_t put(obj_t, int, int);
extern void carry(obj_t, loc_t);
extern void drop(obj_t, loc_t);
extern int atdwrf(loc_t);
extern int setbit(int);
extern bool tstbit(int, int);
extern void set_seed(int32_t);
extern int32_t randrange(int32_t);
extern int score(enum termination);
extern void terminate(enum termination) __attribute__((noreturn));
extern int savefile(FILE *, int32_t);
extern int suspend(void);
extern int resume(void);
extern int restore(FILE *);
extern int initialise(void);
extern phase_codes_t action(command_t);
extern void state_change(obj_t, int);
extern bool is_valid(struct game_t);

void bug(enum bugtype, const char *) __attribute__((__noreturn__));

/* represent an empty command word */
static const command_word_t empty_command_word = {
    .raw = "",
    .id = WORD_EMPTY,
    .type = NO_WORD_TYPE,
};

/* end */