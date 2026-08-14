import React, { FormEvent, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";

type Character = {
  id: string;
  revision: number;
  lifecycle: string;
  name: string;
  summary: string;
  engine_assignment?: string | null;
};

const apiBase = import.meta.env.VITE_API_BASE ?? "http://127.0.0.1:8787/api";

function App() {
  const [characters, setCharacters] = useState<Character[]>([]);
  const [selected, setSelected] = useState<Character | null>(null);
  const [error, setError] = useState("");

  async function refresh() {
    const response = await fetch(`${apiBase}/characters`);
    if (!response.ok) throw new Error(`Unable to load characters (${response.status})`);
    setCharacters(await response.json());
  }

  useEffect(() => { refresh().catch(error => setError(String(error))); }, []);

  async function create(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    const response = await fetch(`${apiBase}/characters`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ name: data.get("name"), summary: data.get("summary") }),
    });
    if (!response.ok) return setError(`Create failed (${response.status})`);
    event.currentTarget.reset();
    await refresh();
  }

  async function save(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selected) return;
    const data = new FormData(event.currentTarget);
    const response = await fetch(`${apiBase}/characters/${selected.id}`, {
      method: "PATCH",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        expected_revision: selected.revision,
        name: data.get("name"),
        summary: data.get("summary"),
        engine_assignment: data.get("engine") ? data.get("engine") : null,
      }),
    });
    if (!response.ok) return setError(`Save failed (${response.status}); reload before retrying.`);
    setSelected(await response.json());
    await refresh();
  }

  async function archive() {
    if (!selected || !confirm(`Archive ${selected.name}?`)) return;
    const response = await fetch(`${apiBase}/characters/${selected.id}`, { method: "DELETE" });
    if (!response.ok) return setError(`Archive failed (${response.status})`);
    setSelected(null);
    await refresh();
  }

  return <main>
    <header><p className="eyebrow">CharacterGen Engine</p><h1>Host console</h1><p>Create, inspect, and revise the persistent cast.</p></header>
    {error && <div className="error" onClick={() => setError("")}>{error}</div>}
    <section className="layout">
      <aside>
        <h2>Characters</h2>
        <div className="list">{characters.map(character =>
          <button className={selected?.id === character.id ? "active" : ""} key={character.id} onClick={() => setSelected(character)}>
            <strong>{character.name}</strong><span>{character.lifecycle}</span>
          </button>)}</div>
        <form onSubmit={create} className="card">
          <h3>Create character</h3>
          <label>Name<input name="name" required /></label>
          <label>Initial context<textarea name="summary" rows={3} /></label>
          <button type="submit">Create stub</button>
        </form>
      </aside>
      <article>
        {!selected ? <div className="empty">Select a character to edit their canonical profile.</div> :
          <form key={`${selected.id}:${selected.revision}`} onSubmit={save} className="editor">
            <div className="editor-heading"><div><span>{selected.lifecycle}</span><h2>{selected.name}</h2></div><code>revision {selected.revision}</code></div>
            <label>Name<input name="name" defaultValue={selected.name} required /></label>
            <label>Canonical summary<textarea name="summary" rows={10} defaultValue={selected.summary} /></label>
            <label>Character engine assignment<input name="engine" defaultValue={selected.engine_assignment ?? ""} placeholder="Use world default" /></label>
            <div className="actions"><button type="submit">Save revision</button><button type="button" className="danger" onClick={archive}>Archive</button></div>
          </form>}
      </article>
    </section>
  </main>;
}

createRoot(document.getElementById("root")!).render(<React.StrictMode><App /></React.StrictMode>);

