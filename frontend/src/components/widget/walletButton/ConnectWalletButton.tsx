import UserAvatar from '@/components/icon/user-avatar-disconnect.svg';
import { useRouter } from 'next/navigation';

export default function ConnectButton() {
  const router = useRouter();
  // return null;
  return (
    <div className="flex justify-center gap-6 items-center">
      <div onClick={() => router.push('/')} className="flex justify-center rounded-full py-3 px-6 bg-[#674DFF] hover:bg-[#9280FF] text-white text-base cursor-pointer">
        Connect Wallet
      </div>
      <UserAvatar className='h-[48px] w-[48px]' />
    </div>
  );
}
