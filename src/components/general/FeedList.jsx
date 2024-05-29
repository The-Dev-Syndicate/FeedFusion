import React, { useState } from 'react';

export default function FeedList() {
    const getThisFromRustLater = ['Feed A', 'Feed B', 'Feed C', 'Feed D']
    return (
        <ul>
            {getThisFromRustLater.map((item, index) => <li className="list-item" key={index}>{item}</li>)}
        </ul>
    );
}