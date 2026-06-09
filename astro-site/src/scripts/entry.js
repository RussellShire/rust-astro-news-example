import React from 'react';
import ReactDOM from 'react-dom/client';
import UserProfile from '../components/UserProfile.jsx';
import FooterRecirc from '../components/FooterRecirc.jsx';

const userProfileTarget = document.getElementById('react-island-user-profile');
if (userProfileTarget) {
    ReactDOM.createRoot(userProfileTarget).render(React.createElement(UserProfile));
}

const footerRecircTarget = document.getElementById('react-island-footer-recirc');
if (footerRecircTarget) {
    ReactDOM.createRoot(footerRecircTarget).render(React.createElement(FooterRecirc));
}
