import Image from 'next/image'
import { Inter } from 'next/font/google'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  return (
    <div className=" min-h-screen bg-[url('/background.webp')] bg-cover">
<div className="navbar bg-slate-50 opacity-95">
  <div className="flex-1">
    <a className="btn btn-ghost text-black font-bold normal-case text-xl">Freefood</a>
  </div>
  <div className="flex-none gap-2">
    <div className="dropdown dropdown-end">
      <label tabIndex={0} className="btn btn-ghost btn-circle avatar">
        <div className="w-10 rounded-full">
          <img src="https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" />
        </div>
      </label>
    </div>
  </div>
</div>

<div className="items-center justify-center pt-10">
<div className="card w-72 bg-primary text-primary-content inset-x-8 mb-10">
  <div className="card-body">
    <h2 className="card-title">Event Title</h2>
    <p>If a dog chews shoes whose shoes does he choose?</p>
  </div>
</div>

<div className="card w-72 bg-primary text-primary-content inset-x-8 mb-10">
  <div className="card-body">
    <h2 className="card-title">Event title</h2>
    <p>If a dog chews shoes whose shoes does he choose?</p>
  </div>
</div>

</div>


      <button className="btn btn-wide bg-slate-200 font-bold fixed inset-x-14 bottom-10">Add Freefood Event</button>
    </div>
  )
}
