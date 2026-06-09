import React, { useState } from 'react';

export default function UserProfile() {
    const [user] = useState("React Client Member");
    return (
        <div style={{ background: '#e0e7ff', color: '#3730a3', padding: '0.5rem 1rem', borderRadius: '20px', fontWeight: '600' }}>
            👤 {user}
        </div>
    );
}
