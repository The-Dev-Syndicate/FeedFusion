import React from 'react';
import AboutCard from '../components/general/About/AboutCard';
import LV from '../res/img/LV.jpeg'

export default function About() {
  const contributors = [
    {
      imageSrc: LV,
      name: 'Lunar Vagabond',
      title: 'Core Developer',
      text: 'a quam. Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi. Duis sapien sem, aliquet nec, commodo eget, consequat quis, neque. Aliquam faucibus, elit ut dictum aliquet, felis nisl adipiscing sapien, sed malesuada diam lacus eget erat. Cras mollis scelerisque nunc. Nullam arcu. Aliquam consequat. Curabitur augue lorem, dapibus quis, laoreet et, pretium ac, nisi. Aenean magna nisl, mollis quis, molestie eu, feugiat in, orci. In hac habitasse platea dictumst.Fusce convallis, mauris imperdiet gravida bibendum, nisl turpis suscipit mauris, sed placerat ipsum urna sed risus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus.',
      imageLeft: true,
    },
    {
      imageSrc: 'path/to/image2.jpg',
      name: 'Jane Smith',
      title: 'UI/UX Designer',
      text: 'Jane is our UI/UX designer with a keen eye for detail...',
      imageLeft: false,
    },
    // Add more contributors as needed
  ];

  return (
    <div className="articles-container">
      {contributors.map((contributor, index) => (
        <AboutCard key={index} {...contributor} />
      ))}
    </div>
  );
}
