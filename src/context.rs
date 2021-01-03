use super::thread::ThreadId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use time::{Duration, Time, Date, OffsetDateTime};

///
/// Public static mutable *cough* agent context. This seems necessary as our code is invoked from
/// the JVM and we need a place to store the temporary mutable data.
///
/// This wouldn't be such a problem if this was a C program, but you know, this is not a C program.
///
lazy_static! {
    static ref STATIC_CONTEXT: AgentContext = AgentContext::new();
}

///
/// Public accessor that provides an abstraction to the global mutable agent state.
///
pub fn static_context() -> &'static AgentContext {
    &STATIC_CONTEXT
}

pub struct AgentContext {
    context: Arc<RwLock<Context>>
}

impl AgentContext {
    pub fn new() -> AgentContext {
        AgentContext {
            context: Arc::new(RwLock::new(Context::new()))
        }
    }

    pub fn thread_start(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                (*ctx).thread_lifetime.insert((*thread_id).clone(), OffsetDateTime::now_utc());
            },
            Err(_) => { /* TODO: Ignore for now */ }
        }
    }

    pub fn thread_end(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = OffsetDateTime::now_utc();
                Some((*ctx).thread_lifetime.remove(thread_id).unwrap_or(now) - now)
            },
            Err(_) => { None /* TODO: Ignore for now */ }
        }
    }

    pub fn monitor_enter(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                (*ctx).monitor_queue.insert((*thread_id).clone(), OffsetDateTime::now_utc());
            },
            Err(_) => {
                // TODO: Ignore this
            }
        }
    }

    pub fn monitor_entered(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = OffsetDateTime::now_utc();
                Some((*ctx).monitor_queue.remove(thread_id).unwrap_or(now) - now)
            },
            Err(_) => { None /* TODO: Ignore for now */ }
        }
    }

    pub fn wait_start(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                (*ctx).thread_wait.insert((*thread_id).clone(), OffsetDateTime::now_utc());
            },
            Err(_) => { /* TODO: Ignore for now */ }
        }
    }

    pub fn wait_end(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = OffsetDateTime::now_utc();
                Some((*ctx).thread_wait.remove(thread_id).unwrap_or(now) - now)
            },
            Err(_) => { None /* TODO: Ignoring for now */ }
        }
    }

    pub fn method_enter(&self, thread_id: &ThreadId) {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = OffsetDateTime::now_utc();

                let new_stack = match (*ctx).method_times.remove(thread_id) {
                    Some(mut thread_stack) => {
                        thread_stack.push(now);
                        thread_stack
                    },
                    None => {
                        let new_vec = vec![ now ];
                        new_vec
                    }
                };

                (*ctx).method_times.insert((*thread_id).clone(), new_stack);
            },
            Err(_) => { /* TODO: Ignoring for now */ }
        }
    }

    pub fn method_exit(&self, thread_id: &ThreadId) -> Option<Duration> {
        match self.context.write() {
            Ok(mut ctx) => {
                let now = OffsetDateTime::now_utc();

                match (*ctx).method_times.get_mut(thread_id) {
                    Some(ref mut thread_stack) => match thread_stack.pop() {
                        Some(time) => Some(time - now),
                        None => None
                    },
                    None => None
                }
            },
            Err(_) => { None /* TODO Ignoring for now */ }
        }
    }
}

pub struct Context {
    pub thread_lifetime: HashMap<ThreadId, OffsetDateTime>,
    pub monitor_queue: HashMap<ThreadId, OffsetDateTime>,
    pub thread_wait: HashMap<ThreadId, OffsetDateTime>,
    pub method_times: HashMap<ThreadId, Vec<OffsetDateTime>>,
    pub method_net_times: HashMap<ThreadId, Vec<OffsetDateTime>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            thread_lifetime: HashMap::new(),
            monitor_queue: HashMap::new(),
            thread_wait: HashMap::new(),
            method_times: HashMap::new(),
            method_net_times: HashMap::new()
        }
    }
}
