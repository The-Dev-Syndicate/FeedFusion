import React from 'react';
import './AboutCard.css';

export default function AboutCard({ imageSrc, name, title, text, imageLeft }) {
  return (
    <div className={`about-card ${imageLeft ? 'image-left' : 'image-right'}`}>
      {imageLeft ? (
        <>
          <img src={imageSrc} alt="About Image" className="about-image" />
          <div className="about-content">
            <h2 className="about-name">{name}</h2>
            <h3 className="about-title">{title}</h3>
            <p className="about-text">{text}</p>
          </div>
        </>
      ) : (
        <>
          <div className="about-content">
            <h2 className="about-name">{name}</h2>
            <h3 className="about-title">{title}</h3>
            <p className="about-text">{text}</p>
          </div>
          <img src={imageSrc} alt="About Image" className="about-image" />
        </>
      )}
    </div>
  );
}
