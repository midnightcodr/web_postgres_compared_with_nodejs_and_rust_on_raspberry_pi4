CREATE TABLE public.posts (
    id integer NOT NULL,
    title character varying NOT NULL,
    body text NOT NULL,
    published boolean DEFAULT false NOT NULL
);