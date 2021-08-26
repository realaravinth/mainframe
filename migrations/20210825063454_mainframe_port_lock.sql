CREATE TABLE IF NOT EXISTS mainframe_port_lock (
    port INTEGER NOT NULL UNIQUE,
    created_at BIGINT NOT NULL 
);
