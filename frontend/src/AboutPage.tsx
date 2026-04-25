import React from 'react';

export default function AboutPage() {
  const members = [
    {
      name: 'Daniel Anindito Nugroho',
      nim: '13524002',
      photo: '/img/foto-daniel.jpeg'
    },
    {
      name: 'Fahd Muhammad Zahid',
      nim: '13524078',
      photo: '/img/foto-fahd.jpeg'
    },
    {
      name: 'Timothy Bernard Soeharto',
      nim: '13524092',
      photo: '/img/foto-soeharto.jpeg'
    },
  ];

  return (
    <div className="flex flex-col items-center py-16 px-4 max-w-4xl mx-auto">
      <h1 className="text-4xl font-black mb-12 text-[#4A453F] text-center">
        Kru Penjelajah
      </h1>

      <div className="w-full grid gap-8 mb-16 md:grid-cols-3">
        {members.map((m, i) => (
          <div
            key={i}
            className="bg-[#FCFAF5] border-2 border-[#4A453F] rounded-2xl p-6 text-center shadow-[6px_6px_0px_0px_rgba(74,69,63,1)] hover:-translate-y-1 hover:shadow-[8px_8px_0px_0px_rgba(74,69,63,1)] transition-all group"
          >
            <div className="w-24 h-24 mx-auto overflow-hidden border-2 border-[#4A453F] rounded-full mb-4 transition-transform group-hover:scale-105">
              <img
                src={m.photo}
                alt={m.name}
                className="w-full h-full object-cover"
              />
            </div>
            <h3 className="font-bold text-xl">{m.name}</h3>
            <p className="font-mono font-bold mt-2 text-[#728C69]">{m.nim}</p>
          </div>
        ))}
      </div>

      <div className="w-full bg-white border-2 border-[#4A453F] p-6 rounded-3xl overflow-hidden">
        <h2 className="text-2xl font-bold mb-6 text-center">Jurnal Visual</h2>
        <div className="w-full aspect-video bg-[#D6CFC1] border-2 border-[#4A453F] rounded-2xl overflow-hidden">
          <iframe
            className="w-full h-full"
            src="https://www.youtube.com/embed/PUqg99S5ApU"
            title="Video Presentasi"
            frameBorder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
            allowFullScreen
          ></iframe>
        </div>
        <p className="text-center mt-4 text-sm font-bold text-[#4A453F] opacity-70">
          Video YouTube Tugas Besar 2 Strategi Algoritma
        </p>
      </div>
    </div>
  );
}
