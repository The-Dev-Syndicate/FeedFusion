import React, { useState } from 'react';

export default function FeedList() {
    const getThisFromRustLater = ['Feed A', 'Feed B', 'Feed C', 'Feed D']
    return (
        <>
        <h1>Feeds</h1>
        <ul>
            {getThisFromRustLater.map((item) => <li class="list-item">{item}</li>)}
        </ul>
        </>
    );
}