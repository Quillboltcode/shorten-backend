--
-- PostgreSQL database dump
--

-- Dumped from database version 16.8 (Debian 16.8-1.pgdg120+1)
-- Dumped by pg_dump version 16.8 (Debian 16.8-1.pgdg120+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO "user";

--
-- Name: account; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.account (
    user_id integer NOT NULL,
    name character varying,
    email character varying NOT NULL,
    password_hash character varying NOT NULL
);


ALTER TABLE public.account OWNER TO "user";

--
-- Name: account_user_id_seq; Type: SEQUENCE; Schema: public; Owner: user
--

CREATE SEQUENCE public.account_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.account_user_id_seq OWNER TO "user";

--
-- Name: account_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: user
--

ALTER SEQUENCE public.account_user_id_seq OWNED BY public.account.user_id;


--
-- Name: url_mapping; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.url_mapping (
    short_url character varying(10) NOT NULL,
    alias character varying(255),
    long_url character varying NOT NULL,
    creation_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    expiration_date timestamp without time zone,
    user_id integer,
    click_count integer DEFAULT 0
);


ALTER TABLE public.url_mapping OWNER TO "user";

--
-- Name: account user_id; Type: DEFAULT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.account ALTER COLUMN user_id SET DEFAULT nextval('public.account_user_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: user
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
\.


--
-- Data for Name: account; Type: TABLE DATA; Schema: public; Owner: user
--

COPY public.account (user_id, name, email, password_hash) FROM stdin;
1	\N	alice@example.com	hashedpassword1
2	\N	bob@example.com	hashedpassword2
\.


--
-- Data for Name: url_mapping; Type: TABLE DATA; Schema: public; Owner: user
--

COPY public.url_mapping (short_url, alias, long_url, creation_date, expiration_date, user_id, click_count) FROM stdin;
abc123	example	https://example.com	2025-03-16 15:49:24.925259	\N	1	0
xyz789	rust	https://rust-lang.org	2025-03-16 15:49:24.925261	\N	2	0
free456	\N	https://opensource.org	2025-03-16 15:49:24.925261	\N	\N	0
1q32An	\N	https://example.com	2025-03-16 16:36:55.244343	2025-04-15 16:36:55.244344	\N	0
\.


--
-- Name: account_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: user
--

SELECT pg_catalog.setval('public.account_user_id_seq', 2, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: account account_email_key; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_email_key UNIQUE (email);


--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_pkey PRIMARY KEY (user_id);


--
-- Name: url_mapping url_mapping_alias_key; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.url_mapping
    ADD CONSTRAINT url_mapping_alias_key UNIQUE (alias);


--
-- Name: url_mapping url_mapping_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.url_mapping
    ADD CONSTRAINT url_mapping_pkey PRIMARY KEY (short_url);


--
-- Name: url_mapping url_mapping_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.url_mapping
    ADD CONSTRAINT url_mapping_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.account(user_id) ON DELETE SET NULL;


--
-- PostgreSQL database dump complete
--

