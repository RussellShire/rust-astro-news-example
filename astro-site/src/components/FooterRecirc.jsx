import React, { useState } from 'react';

export default function FooterRecirc() {
    const [likes, setLikes] = useState(0);
    return (
        <div style={{ marginTop: '2rem', background: '#fef2f2', padding: '1rem', borderRadius: '8px', border: '1px solid #fee2e2' }}>
            <h4>Dynamic Island Recommendations</h4>
            <button onClick={() => setLikes(likes + 1)} style={{ background: '#ef4444', color: 'white', border: 'none', padding: '0.4rem 0.8rem', borderRadius: '4px', cursor: 'pointer' }}>
                👍 Upvote Feed ({likes})
            </button>
        </div>
    );
}
